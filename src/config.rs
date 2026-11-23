//! Runtime configuration.
//!
//! Configuration is read once, at startup. Handlers receive the resulting
//! values through `ApiContext`, not by reading environment variables on every
//! request.

use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_address: SocketAddr,
}

impl Config {
    /// Read configuration from the environment.
    ///
    /// `?` returns early with a `ConfigError` if the value is malformed.
    pub fn from_env() -> Result<Self, ConfigError> {
        let value = std::env::var("WOW_API_BIND").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

        let bind_address = value
            .parse()
            .map_err(|_| ConfigError::InvalidBindAddress { value })?;

        Ok(Self { bind_address })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("WOW_API_BIND must be a socket address like 127.0.0.1:8080, got {value:?}")]
    InvalidBindAddress { value: String },
}
