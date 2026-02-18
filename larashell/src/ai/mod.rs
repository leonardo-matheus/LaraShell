//! AI Module for LaraShell
//!
//! This module provides AI-powered autocomplete functionality using Azure OpenAI.
//! It includes configuration management, HTTP client, caching, and rate limiting.

pub mod autocomplete;
pub mod client;
pub mod config;

pub use autocomplete::AutocompleteEngine;
pub use client::AzureOpenAiClient;
pub use config::AiConfig;
