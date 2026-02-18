//! Autocomplete Engine
//!
//! Provides AI-powered command autocomplete with caching, debouncing, and rate limiting.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio::time::sleep;

use super::client::{AzureOpenAiClient, ChatMessage, ClientError};
use super::config::AiConfig;

/// A cached suggestion with expiration time.
#[derive(Debug, Clone)]
struct CachedSuggestion {
    suggestions: Vec<String>,
    expires_at: Instant,
}

/// Rate limiter tracking request timestamps.
struct RateLimiter {
    timestamps: Vec<Instant>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: u32) -> Self {
        Self {
            timestamps: Vec::new(),
            max_requests,
            window: Duration::from_secs(60), // 1 minute window
        }
    }

    /// Checks if a request can be made. Returns true if allowed.
    fn allow_request(&mut self) -> bool {
        let now = Instant::now();

        // Remove timestamps outside the window
        self.timestamps.retain(|&ts| now.duration_since(ts) < self.window);

        if self.timestamps.len() < self.max_requests as usize {
            self.timestamps.push(now);
            true
        } else {
            false
        }
    }

    /// Returns the time to wait before the next request is allowed.
    fn time_until_available(&self) -> Option<Duration> {
        if self.timestamps.len() < self.max_requests as usize {
            return None;
        }

        let now = Instant::now();
        self.timestamps
            .first()
            .map(|&oldest| {
                let elapsed = now.duration_since(oldest);
                if elapsed < self.window {
                    self.window - elapsed
                } else {
                    Duration::ZERO
                }
            })
    }
}

/// Suggestion result from the autocomplete engine.
#[derive(Debug, Clone)]
pub struct SuggestionResult {
    pub suggestions: Vec<String>,
    pub from_cache: bool,
    pub is_fallback: bool,
}

/// The main autocomplete engine.
pub struct AutocompleteEngine {
    config: AiConfig,
    client: Option<AzureOpenAiClient>,
    cache: Arc<Mutex<HashMap<String, CachedSuggestion>>>,
    rate_limiter: Arc<Mutex<RateLimiter>>,
    last_request_time: Arc<Mutex<Option<Instant>>>,
    pending_request: Arc<Mutex<Option<String>>>,
}

impl AutocompleteEngine {
    /// Creates a new autocomplete engine with the given configuration.
    pub fn new(config: AiConfig) -> Self {
        let client = if config.enabled {
            AzureOpenAiClient::new(&config).ok()
        } else {
            None
        };

        Self {
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new(config.max_requests_per_minute))),
            cache: Arc::new(Mutex::new(HashMap::new())),
            last_request_time: Arc::new(Mutex::new(None)),
            pending_request: Arc::new(Mutex::new(None)),
            client,
            config,
        }
    }

    /// Gets suggestions for the given input with debouncing.
    pub async fn get_suggestions(&self, input: &str) -> Result<SuggestionResult, ClientError> {
        if !self.config.enabled || input.trim().is_empty() {
            return Ok(SuggestionResult {
                suggestions: Vec::new(),
                from_cache: false,
                is_fallback: false,
            });
        }

        // Check cache first
        if let Some(cached) = self.get_from_cache(input) {
            return Ok(SuggestionResult {
                suggestions: cached,
                from_cache: true,
                is_fallback: false,
            });
        }

        // Apply debounce
        {
            let mut pending = self.pending_request.lock();
            *pending = Some(input.to_string());
        }

        sleep(self.config.debounce_duration()).await;

        // Check if this request is still the latest
        {
            let pending = self.pending_request.lock();
            if pending.as_deref() != Some(input) {
                // A newer request has superseded this one
                return Ok(SuggestionResult {
                    suggestions: Vec::new(),
                    from_cache: false,
                    is_fallback: false,
                });
            }
        }

        // Check rate limit
        {
            let mut limiter = self.rate_limiter.lock();
            if !limiter.allow_request() {
                if self.config.use_fallback {
                    return Ok(SuggestionResult {
                        suggestions: self.get_fallback_suggestions(input),
                        from_cache: false,
                        is_fallback: true,
                    });
                }
                return Err(ClientError::RateLimited);
            }
        }

        // Make API request
        match self.fetch_suggestions(input).await {
            Ok(suggestions) => {
                self.add_to_cache(input, suggestions.clone());
                Ok(SuggestionResult {
                    suggestions,
                    from_cache: false,
                    is_fallback: false,
                })
            }
            Err(e) => {
                if self.config.use_fallback {
                    Ok(SuggestionResult {
                        suggestions: self.get_fallback_suggestions(input),
                        from_cache: false,
                        is_fallback: true,
                    })
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Gets suggestions without debouncing (immediate request).
    pub async fn get_suggestions_immediate(&self, input: &str) -> Result<SuggestionResult, ClientError> {
        if !self.config.enabled || input.trim().is_empty() {
            return Ok(SuggestionResult {
                suggestions: Vec::new(),
                from_cache: false,
                is_fallback: false,
            });
        }

        // Check cache first
        if let Some(cached) = self.get_from_cache(input) {
            return Ok(SuggestionResult {
                suggestions: cached,
                from_cache: true,
                is_fallback: false,
            });
        }

        // Check rate limit
        {
            let mut limiter = self.rate_limiter.lock();
            if !limiter.allow_request() {
                if self.config.use_fallback {
                    return Ok(SuggestionResult {
                        suggestions: self.get_fallback_suggestions(input),
                        from_cache: false,
                        is_fallback: true,
                    });
                }
                return Err(ClientError::RateLimited);
            }
        }

        // Make API request
        match self.fetch_suggestions(input).await {
            Ok(suggestions) => {
                self.add_to_cache(input, suggestions.clone());
                Ok(SuggestionResult {
                    suggestions,
                    from_cache: false,
                    is_fallback: false,
                })
            }
            Err(e) => {
                if self.config.use_fallback {
                    Ok(SuggestionResult {
                        suggestions: self.get_fallback_suggestions(input),
                        from_cache: false,
                        is_fallback: true,
                    })
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Fetches suggestions from the Azure OpenAI API.
    async fn fetch_suggestions(&self, input: &str) -> Result<Vec<String>, ClientError> {
        let client = self.client.as_ref().ok_or_else(|| {
            ClientError::ApiError {
                status: reqwest::StatusCode::SERVICE_UNAVAILABLE,
                message: "AI client not initialized".to_string(),
            }
        })?;

        let system_prompt = r#"You are a terminal command autocomplete assistant.
Given a partial command, suggest the most likely completions.
Return only the suggestions, one per line, without explanations.
Suggest 3-5 relevant completions based on common usage patterns.
Consider the context: shell commands, git, npm, cargo, docker, etc."#;

        let user_prompt = format!("Complete this terminal command: {}", input);

        let response = client.prompt(system_prompt, &user_prompt).await?;

        // Parse response into individual suggestions
        let suggestions: Vec<String> = response
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                // Remove common prefixes like "- ", "* ", numbers, etc.
                let line = line.trim_start_matches(|c: char| c == '-' || c == '*' || c == '.' || c.is_numeric());
                line.trim().to_string()
            })
            .filter(|s| !s.is_empty())
            .take(5)
            .collect();

        Ok(suggestions)
    }

    /// Gets a cached suggestion if available and not expired.
    fn get_from_cache(&self, input: &str) -> Option<Vec<String>> {
        let cache = self.cache.lock();
        cache.get(input).and_then(|cached| {
            if Instant::now() < cached.expires_at {
                Some(cached.suggestions.clone())
            } else {
                None
            }
        })
    }

    /// Adds a suggestion to the cache.
    fn add_to_cache(&self, input: &str, suggestions: Vec<String>) {
        let mut cache = self.cache.lock();

        // Evict old entries if cache is full
        if cache.len() >= self.config.max_cache_entries {
            let now = Instant::now();
            cache.retain(|_, v| v.expires_at > now);

            // If still full, remove oldest entries
            if cache.len() >= self.config.max_cache_entries {
                let to_remove: Vec<_> = cache
                    .iter()
                    .take(cache.len() / 4)
                    .map(|(k, _)| k.clone())
                    .collect();
                for key in to_remove {
                    cache.remove(&key);
                }
            }
        }

        cache.insert(
            input.to_string(),
            CachedSuggestion {
                suggestions,
                expires_at: Instant::now() + self.config.cache_ttl(),
            },
        );
    }

    /// Clears the suggestion cache.
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock();
        cache.clear();
    }

    /// Returns fallback suggestions based on common command patterns.
    fn get_fallback_suggestions(&self, input: &str) -> Vec<String> {
        let input_lower = input.to_lowercase();
        let mut suggestions = Vec::new();

        // Common command prefixes and their completions
        let patterns: &[(&str, &[&str])] = &[
            ("git ", &["git status", "git add", "git commit", "git push", "git pull"]),
            ("git c", &["git commit -m \"", "git checkout", "git clone", "git cherry-pick"]),
            ("git p", &["git push", "git pull", "git push origin"]),
            ("git s", &["git status", "git stash", "git show"]),
            ("npm ", &["npm install", "npm run", "npm start", "npm test"]),
            ("npm r", &["npm run", "npm run dev", "npm run build", "npm run test"]),
            ("npm i", &["npm install", "npm init"]),
            ("cargo ", &["cargo build", "cargo run", "cargo test", "cargo check"]),
            ("cargo b", &["cargo build", "cargo build --release"]),
            ("cargo r", &["cargo run", "cargo run --release"]),
            ("cargo t", &["cargo test", "cargo test --", "cargo tree"]),
            ("docker ", &["docker ps", "docker images", "docker run", "docker build"]),
            ("docker p", &["docker ps", "docker pull", "docker push"]),
            ("docker r", &["docker run", "docker rm", "docker rmi"]),
            ("cd ", &["cd ..", "cd ~", "cd -"]),
            ("ls ", &["ls -la", "ls -l", "ls -a"]),
            ("mkdir ", &["mkdir -p"]),
            ("rm ", &["rm -rf", "rm -r"]),
            ("cat ", &["cat"]),
            ("grep ", &["grep -r", "grep -i", "grep -rn"]),
            ("find ", &["find . -name", "find . -type f", "find . -type d"]),
            ("ps ", &["ps aux", "ps -ef"]),
            ("kill ", &["kill -9"]),
            ("chmod ", &["chmod +x", "chmod 755", "chmod 644"]),
            ("ssh ", &["ssh -i"]),
            ("scp ", &["scp -r"]),
            ("curl ", &["curl -X GET", "curl -X POST", "curl -H"]),
            ("wget ", &["wget -O"]),
            ("tar ", &["tar -xvf", "tar -cvf", "tar -xzf"]),
            ("python ", &["python -m", "python -c"]),
            ("pip ", &["pip install", "pip list", "pip freeze"]),
        ];

        for (prefix, completions) in patterns {
            if input_lower.starts_with(prefix) || prefix.starts_with(&input_lower) {
                for completion in *completions {
                    if completion.starts_with(&input_lower) || input_lower.is_empty() {
                        suggestions.push(completion.to_string());
                    }
                }
            }
        }

        // If no specific matches, suggest based on first character
        if suggestions.is_empty() && !input.is_empty() {
            let first_char = input.chars().next().unwrap().to_lowercase().next().unwrap();
            suggestions = match first_char {
                'g' => vec!["git", "grep", "go"],
                'c' => vec!["cd", "cat", "cargo", "curl", "cp"],
                'd' => vec!["docker", "df", "du", "diff"],
                'l' => vec!["ls", "less", "ln"],
                'n' => vec!["npm", "node", "nano"],
                'p' => vec!["ps", "pwd", "pip", "python"],
                'r' => vec!["rm", "rsync"],
                's' => vec!["ssh", "sudo", "scp"],
                't' => vec!["tar", "touch", "tail"],
                'v' => vec!["vim", "vi"],
                _ => vec![],
            }
            .into_iter()
            .filter(|s| s.starts_with(&input_lower))
            .map(String::from)
            .collect();
        }

        suggestions.truncate(5);
        suggestions
    }

    /// Checks if the engine is enabled.
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Gets the current cache size.
    pub fn cache_size(&self) -> usize {
        self.cache.lock().len()
    }

    /// Gets remaining requests in the rate limit window.
    pub fn remaining_requests(&self) -> u32 {
        let limiter = self.rate_limiter.lock();
        let used = limiter.timestamps.len() as u32;
        self.config.max_requests_per_minute.saturating_sub(used)
    }
}

/// Creates a channel for receiving suggestions asynchronously.
pub fn create_suggestion_channel() -> (mpsc::Sender<String>, mpsc::Receiver<SuggestionResult>) {
    let (input_tx, mut input_rx) = mpsc::channel::<String>(32);
    let (result_tx, result_rx) = mpsc::channel::<SuggestionResult>(32);

    tokio::spawn(async move {
        let config = AiConfig::default();
        let engine = AutocompleteEngine::new(config);

        while let Some(input) = input_rx.recv().await {
            match engine.get_suggestions(&input).await {
                Ok(result) => {
                    let _ = result_tx.send(result).await;
                }
                Err(_) => {
                    let _ = result_tx
                        .send(SuggestionResult {
                            suggestions: Vec::new(),
                            from_cache: false,
                            is_fallback: false,
                        })
                        .await;
                }
            }
        }
    });

    (input_tx, result_rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(3);
        assert!(limiter.allow_request());
        assert!(limiter.allow_request());
        assert!(limiter.allow_request());
        assert!(!limiter.allow_request());
    }

    #[test]
    fn test_fallback_suggestions() {
        let config = AiConfig::default();
        let engine = AutocompleteEngine::new(config);

        let suggestions = engine.get_fallback_suggestions("git c");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.starts_with("git c")));
    }

    #[test]
    fn test_cache() {
        let config = AiConfig::default();
        let engine = AutocompleteEngine::new(config);

        engine.add_to_cache("test", vec!["test1".to_string(), "test2".to_string()]);
        let cached = engine.get_from_cache("test");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 2);
    }

    #[test]
    fn test_clear_cache() {
        let config = AiConfig::default();
        let engine = AutocompleteEngine::new(config);

        engine.add_to_cache("test", vec!["test1".to_string()]);
        assert_eq!(engine.cache_size(), 1);

        engine.clear_cache();
        assert_eq!(engine.cache_size(), 0);
    }

    #[test]
    fn test_disabled_engine() {
        let mut config = AiConfig::default();
        config.enabled = false;
        let engine = AutocompleteEngine::new(config);

        assert!(!engine.is_enabled());
    }
}
