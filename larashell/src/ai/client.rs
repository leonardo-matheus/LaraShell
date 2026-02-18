//! Azure OpenAI HTTP Client
//!
//! Provides an async HTTP client for communicating with Azure OpenAI API.

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::config::AiConfig;

/// Error types for the Azure OpenAI client.
#[derive(Debug)]
pub enum ClientError {
    /// HTTP request failed.
    RequestFailed(reqwest::Error),
    /// API returned an error response.
    ApiError { status: StatusCode, message: String },
    /// Failed to parse response.
    ParseError(String),
    /// Request timeout.
    Timeout,
    /// Rate limit exceeded.
    RateLimited,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::RequestFailed(e) => write!(f, "Request failed: {}", e),
            ClientError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            ClientError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ClientError::Timeout => write!(f, "Request timeout"),
            ClientError::RateLimited => write!(f, "Rate limit exceeded"),
        }
    }
}

impl std::error::Error for ClientError {}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ClientError::Timeout
        } else {
            ClientError::RequestFailed(err)
        }
    }
}

/// Message in the chat completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Request body for Azure OpenAI chat completions.
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
}

/// Choice in the chat completion response.
#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

/// Response from Azure OpenAI chat completions.
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

/// Azure OpenAI client for making API requests.
pub struct AzureOpenAiClient {
    client: Client,
    api_key: String,
    endpoint: String,
    max_tokens: u32,
    temperature: f32,
}

impl AzureOpenAiClient {
    /// Creates a new Azure OpenAI client from configuration.
    pub fn new(config: &AiConfig) -> Result<Self, ClientError> {
        let client = Client::builder()
            .timeout(config.timeout())
            .build()
            .map_err(ClientError::RequestFailed)?;

        Ok(Self {
            client,
            api_key: config.get_api_key().to_string(),
            endpoint: config.get_endpoint().to_string(),
            max_tokens: config.max_tokens,
            temperature: config.temperature,
        })
    }

    /// Creates a client with custom timeout.
    pub fn with_timeout(config: &AiConfig, timeout: Duration) -> Result<Self, ClientError> {
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(ClientError::RequestFailed)?;

        Ok(Self {
            client,
            api_key: config.get_api_key().to_string(),
            endpoint: config.get_endpoint().to_string(),
            max_tokens: config.max_tokens,
            temperature: config.temperature,
        })
    }

    /// Sends a chat completion request and returns the response text.
    pub async fn complete(&self, messages: Vec<ChatMessage>) -> Result<String, ClientError> {
        let request_body = ChatCompletionRequest {
            messages,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
        };

        let response = self
            .client
            .post(&self.endpoint)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();

        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(ClientError::RateLimited);
        }

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ClientError::ApiError {
                status,
                message: error_text,
            });
        }

        let completion: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| ClientError::ParseError(e.to_string()))?;

        completion
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| ClientError::ParseError("No choices in response".to_string()))
    }

    /// Sends a simple prompt and returns the completion.
    pub async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String, ClientError> {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];

        self.complete(messages).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = AiConfig::default();
        let client = AzureOpenAiClient::new(&config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Hello"));
    }
}
