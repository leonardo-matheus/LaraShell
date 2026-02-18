//! AI Configuration Module
//!
//! Handles configuration for the AI autocomplete feature with TOML support.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Azure OpenAI API credentials and settings (hardcoded as per plan).
pub const AZURE_API_KEY: &str = "83gUFP0agxEMOT5gvipaHoeRUTpFUyQTYLRFOrmxYfNX0wg3J0wAJQQJ99CAACHYHv6XJ3w3AAAAACOGeoCc";
pub const AZURE_ENDPOINT: &str = "https://conta-ma6t6uyn-eastus2.openai.azure.com/openai/deployments/gpt-4.1/chat/completions?api-version=2025-01-01-preview";
pub const AZURE_MODEL: &str = "gpt-4.1";

/// Configuration for the AI autocomplete feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    /// Whether AI autocomplete is enabled.
    pub enabled: bool,

    /// API key for Azure OpenAI (overrides hardcoded value if set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// API endpoint URL (overrides hardcoded value if set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// Model name to use.
    pub model: String,

    /// Debounce delay in milliseconds before sending a request.
    pub debounce_ms: u64,

    /// Maximum requests per minute (rate limiting).
    pub max_requests_per_minute: u32,

    /// Cache time-to-live in seconds.
    pub cache_ttl_secs: u64,

    /// Maximum number of cached suggestions.
    pub max_cache_entries: usize,

    /// Request timeout in seconds.
    pub timeout_secs: u64,

    /// Maximum tokens for completion response.
    pub max_tokens: u32,

    /// Temperature for response generation (0.0 - 2.0).
    pub temperature: f32,

    /// Whether to use fallback suggestions when API fails.
    pub use_fallback: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_key: None,
            endpoint: None,
            model: AZURE_MODEL.to_string(),
            debounce_ms: 300,
            max_requests_per_minute: 50,
            cache_ttl_secs: 300, // 5 minutes
            max_cache_entries: 1000,
            timeout_secs: 10,
            max_tokens: 100,
            temperature: 0.3,
            use_fallback: true,
        }
    }
}

impl AiConfig {
    /// Creates a new AiConfig from a TOML string.
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Serializes the config to a TOML string.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Returns the API key to use (config override or hardcoded).
    pub fn get_api_key(&self) -> &str {
        self.api_key.as_deref().unwrap_or(AZURE_API_KEY)
    }

    /// Returns the endpoint to use (config override or hardcoded).
    pub fn get_endpoint(&self) -> &str {
        self.endpoint.as_deref().unwrap_or(AZURE_ENDPOINT)
    }

    /// Returns the debounce duration.
    pub fn debounce_duration(&self) -> Duration {
        Duration::from_millis(self.debounce_ms)
    }

    /// Returns the cache TTL duration.
    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache_ttl_secs)
    }

    /// Returns the request timeout duration.
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AiConfig::default();
        assert!(config.enabled);
        assert_eq!(config.debounce_ms, 300);
        assert_eq!(config.max_requests_per_minute, 50);
    }

    #[test]
    fn test_toml_roundtrip() {
        let config = AiConfig::default();
        let toml_str = config.to_toml().unwrap();
        let parsed = AiConfig::from_toml(&toml_str).unwrap();
        assert_eq!(config.enabled, parsed.enabled);
        assert_eq!(config.debounce_ms, parsed.debounce_ms);
    }

    #[test]
    fn test_api_key_override() {
        let mut config = AiConfig::default();
        assert_eq!(config.get_api_key(), AZURE_API_KEY);

        config.api_key = Some("custom-key".to_string());
        assert_eq!(config.get_api_key(), "custom-key");
    }
}
