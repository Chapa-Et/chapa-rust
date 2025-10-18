//! # Config Module
//!
//! This module provides a configuration mechanism for interacting with the Chapa API. It includes
//! settings for API keys, base URLs, api versions, headers, and timeouts. The `ChapaConfig` struct encapsulates
//! all necessary configuration details required to make requests to the Chapa API.
//!
//! ## Overview
//!
//! The `ChapaConfig` struct is the main configuration structure, and it is built using the
//! `ChapaConfigBuilder` struct, which implements the builder pattern. This allows for a flexible
//! and step-by-step configuration process. Default values are provided for most fields, and
//! environment variables can be used to set the API key.
//!
//! ## Features
//!
//! - **API Key**: Required for authentication with the Chapa API.
//! - **Base URL**: The endpoint for the Chapa API, defaulting to `https://api.chapa.co`.
//! - **Version**: The version of the API to use, defaulting to `v1`.
//! - **Default Headers**: Headers included in every API request, such as `Content-Type`.
//! - **Timeout**: Request timeout duration, defaulting to 30 seconds.
//!
//! ## Example Usage
//!
//! ```rust
//! use chapa_rust::config::ChapaConfig;
//! use std::time::Duration;
//!
//! // Building a default configuration
//! let default_config = ChapaConfig::builder()
//!     .build()
//!     .expect("Failed to build default config");
//!
//! println!("Default Base URL: {}", default_config.base_url);
//!
//! // Customizing the configuration
//! let custom_config = ChapaConfig::builder()
//!     .base_url("http://localhost:8080/dev")
//!     .timeout(Duration::from_secs(10))
//!     .api_key("my-custom-api-key")
//!     .add_header("X-Custom-Header", "CustomValue")
//!     .build()
//!     .expect("Failed to build custom config");
//!
//! println!("Custom Base URL: {}", custom_config.base_url);
//! println!("Custom Timeout: {} seconds", custom_config.timeout.as_secs());
//!
//! ```
//!
//! ## Testing
//!
//! The module includes unit tests to verify the default configuration and the builder pattern:
//!
//! - `test_default_config`: Ensures the default configuration is correctly initialized.
//! - `test_builder_pattern`: Validates the builder pattern for creating custom configurations.
//!
//! ## Notes
//!
//! - The API key can be set using the `CHAPA_API_PUBLIC_KEY` environment variable.
//! - If the API key is not provided, a placeholder value will be used, and an error will be returned
//!   when attempting to build the configuration.
use std::collections::HashMap;
use std::time::Duration;

const PLACEHOLDER_API_KEY: &str = "placeholder_api_key";

/// The `ChapaConfig` struct provides a centralized configuration mechanism for
/// interacting with an external API. It encapsulates essential settings such as
/// the API key, base URL, default headers, and request timeout duration.
#[derive(Debug, Clone)]
pub struct ChapaConfig {
    /// The API key required for authentication with the external service.
    /// [more](https://developer.chapa.co/dashboard/quick-start) on api key.
    pub api_key: String,
    /// The base URL for the external API, usually it's `https://api.chapa.co`
    pub base_url: String,
    /// The version of the API to be used.
    pub version: String,
    /// Default headers to be included in every API request.
    pub default_headers: HashMap<String, String>,
    /// Request timeout duration. default to 30s.
    pub timeout: Duration,
}

impl ChapaConfig {
    /// Gives an instance of `ChapaConfigBuilder` for the `ChapaConfig`.
    pub fn builder() -> ChapaConfigBuilder {
        ChapaConfigBuilder::new()
    }
}

/// The `ChapaConfigBuilder` struct implements the builder pattern for
/// constructing a `ChapaConfig` instance. It allows for step-by-step
/// configuration of the API client.
#[derive(Debug, Clone)]
pub struct ChapaConfigBuilder {
    /// The API key required for authentication with the external service.
    /// [click](https://developer.chapa.co/dashboard/quick-start) to read more on api key.
    api_key: Option<String>,
    /// The base URL for the external API, usually it's `https://api.chapa.co`
    base_url: Option<String>,
    /// The version of the API to be used. default is `v1`.
    version: Option<String>,
    /// Default headers to be included in every API request.
    default_headers: HashMap<String, String>,
    /// Request timeout duration. default to 30s.
    timeout: Option<Duration>,
}

impl ChapaConfigBuilder {
    /// Creates a new instance of `ChapaConfigBuilder` with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a custom base URL for the API.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the API version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets the API key.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Sets the request timeout duration.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Adds a specific header key/value pair.
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    /// Finalizes the configuration and validates it before use.
    pub fn build(self) -> Result<ChapaConfig, String> {
        if self.api_key.is_none() || self.api_key == Some(PLACEHOLDER_API_KEY.to_string()) {
            // TODO: use app error
            return Err("API Key is required but not set. Please set it using the CHAPA_API_PUBLIC_KEY environment variable or via the builder's api_key() method.".to_string());
        }

        Ok(ChapaConfig {
            api_key: self.api_key.unwrap(),
            base_url: self.base_url.unwrap(),
            version: self.version.unwrap(),
            default_headers: self.default_headers,
            timeout: self.timeout.unwrap(),
        })
    }
}

impl Default for ChapaConfigBuilder {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let default_api_key = std::env::var("CHAPA_API_PUBLIC_KEY")
            .unwrap_or_else(|_| PLACEHOLDER_API_KEY.to_string());

        Self {
            api_key: Some(default_api_key),
            base_url: Some("https://api.chapa.co".to_string()),
            version: Some("v1".to_string()),
            default_headers: headers,
            timeout: Some(Duration::from_secs(30)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_default_config() {
        // consider the warnings from the env::set_var() and env::remove_var() functions about the unsafe usage.
        // if the tests are run in parallel, it may cause issues.
        let config: ChapaConfig;
        unsafe {
            env::set_var("CHAPA_API_PUBLIC_KEY", "test_api_key_123");
            config = ChapaConfig::builder()
                .build()
                .expect("Failed to build config");
            env::remove_var("CHAPA_API_PUBLIC_KEY");
        }
        assert_eq!(config.base_url, "https://api.chapa.co");
        assert_eq!(config.version, "v1");
        assert_eq!(config.timeout.as_secs(), 30);
        assert!(config.api_key.contains("placeholder") || !config.api_key.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let config = ChapaConfig::builder()
            .base_url("http://localhost:8080/dev")
            .version("v2")
            .timeout(Duration::from_secs(5))
            .api_key("my-secret-key-123")
            .add_header("X-Client-ID", "chapa-cli")
            .build()
            .expect("Failed to build config");

        assert_eq!(config.base_url, "http://localhost:8080/dev");
        assert_eq!(config.timeout.as_secs(), 5);
        assert_eq!(
            config.default_headers.get("X-Client-ID"),
            Some(&"chapa-cli".to_string())
        );
    }
}
