//! Runtime configuration.
//!
//! Configuration is read once, at startup. Handlers receive the resulting
//! values through `ApiContext`, not by reading environment variables on every
//! request.

use std::net::SocketAddr;

/// A Battle.net region.
///
/// Rust concept: an enum makes invalid regions unrepresentable. Once a value is
/// a `Region`, it is guaranteed to be one of these five variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    Us,
    Eu,
    Kr,
    Tw,
    Cn,
}

impl Region {
    /// Parse a region from its short name (`us`, `eu`, ...).
    pub fn parse(value: &str) -> Result<Self, ConfigError> {
        match value.to_ascii_lowercase().as_str() {
            "us" => Ok(Self::Us),
            "eu" => Ok(Self::Eu),
            "kr" => Ok(Self::Kr),
            "tw" => Ok(Self::Tw),
            "cn" => Ok(Self::Cn),
            other => Err(ConfigError::InvalidRegion {
                value: other.to_string(),
            }),
        }
    }

    /// The short name used in namespaces such as `static-us`.
    pub fn slug(self) -> &'static str {
        match self {
            Self::Us => "us",
            Self::Eu => "eu",
            Self::Kr => "kr",
            Self::Tw => "tw",
            Self::Cn => "cn",
        }
    }

    /// Base URL for game-data and profile requests.
    pub fn api_base_url(self) -> &'static str {
        match self {
            Self::Us => "https://us.api.blizzard.com",
            Self::Eu => "https://eu.api.blizzard.com",
            Self::Kr => "https://kr.api.blizzard.com",
            Self::Tw => "https://tw.api.blizzard.com",
            // China uses a separate gateway.
            Self::Cn => "https://gateway.battlenet.com.cn",
        }
    }

    /// OAuth token endpoint.
    pub fn token_url(self) -> &'static str {
        match self {
            // China uses a separate OAuth host and separate credentials.
            Self::Cn => "https://oauth.battlenet.com.cn/token",
            _ => "https://oauth.battle.net/token",
        }
    }

    /// Namespace for static game data (items, spells, ...).
    pub fn static_namespace(self) -> String {
        format!("static-{}", self.slug())
    }

    /// Namespace for character profiles.
    pub fn profile_namespace(self) -> String {
        format!("profile-{}", self.slug())
    }

    /// Namespace for changing game data such as the realm index.
    pub fn dynamic_namespace(self) -> String {
        format!("dynamic-{}", self.slug())
    }
}

/// Everything the server needs to know that comes from the environment.
#[derive(Clone, Debug)]
pub struct Config {
    pub bind_address: SocketAddr,
    pub bnet_client_id: String,
    pub bnet_client_secret: String,
    pub region: Region,
    pub locale: String,
}

impl Config {
    /// Read configuration from the environment.
    ///
    /// `?` returns early with a `ConfigError` if a required value is missing or
    /// malformed.
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind_value =
            std::env::var("WOW_API_BIND").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let bind_address = bind_value
            .parse()
            .map_err(|_| ConfigError::InvalidBindAddress {
                value: bind_value.clone(),
            })?;

        let region = match std::env::var("BNET_REGION") {
            Ok(value) => Region::parse(&value)?,
            Err(_) => Region::Us,
        };

        Ok(Self {
            bind_address,
            bnet_client_id: required("BNET_CLIENT_ID")?,
            bnet_client_secret: required("BNET_CLIENT_SECRET")?,
            region,
            locale: std::env::var("BNET_LOCALE").unwrap_or_else(|_| "en_US".to_string()),
        })
    }
}

fn required(name: &'static str) -> Result<String, ConfigError> {
    std::env::var(name).map_err(|_| ConfigError::MissingVar(name))
}

/// A configuration problem.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    MissingVar(&'static str),

    #[error("WOW_API_BIND must be a socket address like 127.0.0.1:8080, got {value:?}")]
    InvalidBindAddress { value: String },

    #[error("BNET_REGION must be one of us, eu, kr, tw, cn; got {value:?}")]
    InvalidRegion { value: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_parsing_is_case_insensitive() {
        assert_eq!(Region::parse("US").unwrap(), Region::Us);
        assert_eq!(Region::parse("eu").unwrap(), Region::Eu);
    }

    #[test]
    fn unknown_region_is_rejected() {
        assert!(matches!(
            Region::parse("moon"),
            Err(ConfigError::InvalidRegion { .. })
        ));
    }

    #[test]
    fn namespaces_use_the_region_slug() {
        assert_eq!(Region::Eu.static_namespace(), "static-eu");
        assert_eq!(Region::Eu.profile_namespace(), "profile-eu");
        assert_eq!(Region::Eu.dynamic_namespace(), "dynamic-eu");
    }

    #[test]
    fn regional_urls_are_correct() {
        assert_eq!(Region::Us.api_base_url(), "https://us.api.blizzard.com");
        assert_eq!(Region::Us.token_url(), "https://oauth.battle.net/token");
    }
}
