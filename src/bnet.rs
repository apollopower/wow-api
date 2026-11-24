//! A small Battle.net API client.
//!
//! It handles the OAuth client-credentials flow, caches the access token, and
//! fetches typed JSON from the regional API host.

use std::sync::Arc;
use std::time::{Duration, Instant};

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::config::{Config, Region};

/// A Battle.net request failed.
#[derive(Debug, thiserror::Error)]
pub enum BnetError {
    #[error("Battle.net request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Battle.net returned {status} for {url}: {body}")]
    Api {
        status: reqwest::StatusCode,
        url: String,
        body: String,
    },

    #[error("Battle.net token response did not contain an access token")]
    MissingToken,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default = "default_expires_in")]
    expires_in: u64,
}

fn default_expires_in() -> u64 {
    // If Blizzard omits this, assume a conservative five minutes.
    300
}

struct CachedToken {
    value: String,
    expires_at: Instant,
}

/// Thread-safe client shared by every request handler.
///
/// Rust concept: `Clone` is cheap because the expensive parts live behind
/// `Arc`. Cloning a `BnetClient` gives you another handle to the same HTTP
/// connection pool and the same token cache.
#[derive(Clone)]
pub struct BnetClient {
    http: reqwest::Client,
    config: Arc<Config>,
    token: Arc<Mutex<Option<CachedToken>>>,
}

impl BnetClient {
    pub fn new(config: Config) -> Result<Self, BnetError> {
        let http = reqwest::Client::builder()
            .user_agent(concat!("wow-api/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(15))
            .build()?;

        Ok(Self {
            http,
            config: Arc::new(config),
            token: Arc::new(Mutex::new(None)),
        })
    }

    pub fn region(&self) -> Region {
        self.config.region
    }

    pub fn locale(&self) -> &str {
        &self.config.locale
    }

    /// Return a valid access token, fetching a new one if necessary.
    ///
    /// We hold the mutex across the network call on purpose: if several
    /// requests arrive with no cached token, we want one token request, not
    /// one per request.
    async fn access_token(&self) -> Result<String, BnetError> {
        let mut cached = self.token.lock().await;

        if let Some(token) = cached.as_ref()
            && Instant::now() < token.expires_at
        {
            return Ok(token.value.clone());
        }

        let token_url = self.config.region.token_url();
        let response = self
            .http
            .post(token_url)
            .basic_auth(
                &self.config.bnet_client_id,
                Some(&self.config.bnet_client_secret),
            )
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(BnetError::Api {
                status,
                url: token_url.to_string(),
                body,
            });
        }

        let token: TokenResponse = response.json().await?;
        if token.access_token.is_empty() {
            return Err(BnetError::MissingToken);
        }

        // Refresh a minute early so a request cannot race the expiry.
        let ttl = token.expires_in.saturating_sub(60).max(30);
        let value = token.access_token;
        *cached = Some(CachedToken {
            value: value.clone(),
            expires_at: Instant::now() + Duration::from_secs(ttl),
        });

        Ok(value)
    }

    /// GET a JSON document from the regional API host.
    async fn get<T: DeserializeOwned>(&self, path: &str, namespace: &str) -> Result<T, BnetError> {
        let token = self.access_token().await?;
        let url = format!("{}{}", self.config.region.api_base_url(), path);

        let response = self
            .http
            .get(&url)
            .query(&[
                ("namespace", namespace),
                ("locale", self.config.locale.as_str()),
            ])
            .bearer_auth(token)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(BnetError::Api { status, url, body });
        }

        Ok(response.json().await?)
    }

    /// Character summary from the profile namespace.
    ///
    /// `realm` is a realm slug such as `tichondrius`; `name` is the character
    /// name. Battle.net character names are case-insensitive.
    pub async fn character(&self, realm: &str, name: &str) -> Result<CharacterSummary, BnetError> {
        let path = format!("/profile/wow/character/{realm}/{name}");
        self.get(&path, &self.config.region.profile_namespace())
            .await
    }

    /// Item details from the static game-data namespace.
    pub async fn item(&self, id: u64) -> Result<Item, BnetError> {
        let path = format!("/data/wow/item/{id}");
        self.get(&path, &self.config.region.static_namespace())
            .await
    }
}

// ---------------------------------------------------------------------------
// Response models
//
// Serde ignores JSON fields that are not mentioned here, so these structs are
// deliberately a subset of what Battle.net returns.
// ---------------------------------------------------------------------------

/// A character's public profile summary.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CharacterSummary {
    pub id: u64,
    pub name: String,
    pub level: u32,
    pub realm: RealmRef,
    #[serde(default)]
    pub race: Option<NamedRef>,
    pub character_class: NamedRef,
    #[serde(default)]
    pub active_spec: Option<NamedRef>,
    #[serde(default)]
    pub faction: Option<TypedName>,
    #[serde(default)]
    pub achievement_points: u64,
    #[serde(default)]
    pub average_item_level: u32,
    #[serde(default)]
    pub equipped_item_level: u32,
    #[serde(default)]
    pub last_login_timestamp: Option<i64>,
}

/// A reference with a numeric id and a display name.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct NamedRef {
    pub id: u64,
    pub name: String,
}

/// A realm reference, which includes the URL-friendly `slug`.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct RealmRef {
    pub id: u64,
    pub slug: String,
    pub name: String,
}

/// Battle.net's common `{ "type": "...", "name": "..." }` shape.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct TypedName {
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
}

/// An item from the static game-data API.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub quality: TypedName,
    pub level: u32,
    #[serde(default)]
    pub required_level: u32,
    pub item_class: NamedRef,
    pub item_subclass: NamedRef,
    #[serde(default)]
    pub inventory_type: Option<TypedName>,
    #[serde(default)]
    pub purchase_price: u64,
    #[serde(default)]
    pub sell_price: u64,
    #[serde(default)]
    pub max_count: u64,
    #[serde(default)]
    pub is_equippable: bool,
    #[serde(default)]
    pub is_stackable: bool,
}
