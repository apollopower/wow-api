//! Shared helpers for integration tests.
//!
//! `tests/common/mod.rs` is not compiled as its own test crate. Each test file
//! opts in with `mod common;`, which is the standard Cargo pattern for shared
//! integration-test setup.

use std::sync::Arc;

use async_trait::async_trait;
use dropshot::test_util::{LogContext, TestContext};
use dropshot::{ConfigDropshot, ConfigLogging, ConfigLoggingLevel};

use wow_api::api;
use wow_api::bnet::{BnetApi, BnetError, CharacterSummary, Item, NamedRef, RealmRef, TypedName};
use wow_api::context::ApiContext;

/// An in-memory Battle.net client for tests.
///
/// It never touches the network. The character response is deterministic, so
/// tests can assert on exact values.
pub struct FakeBnet;

#[async_trait]
impl BnetApi for FakeBnet {
    async fn character(&self, realm: &str, name: &str) -> Result<CharacterSummary, BnetError> {
        if name == "Missing" {
            return Err(BnetError::Api {
                status: http::StatusCode::NOT_FOUND,
                url: format!("https://example.invalid/profile/wow/character/{realm}/{name}"),
                body: r#"{"message":"Character not found"}"#.to_string(),
            });
        }

        Ok(CharacterSummary {
            id: 123,
            name: name.to_string(),
            level: 80,
            realm: RealmRef {
                id: 1,
                slug: realm.to_string(),
                name: "Tichondrius".to_string(),
            },
            race: Some(NamedRef {
                id: 1,
                name: "Human".to_string(),
            }),
            character_class: NamedRef {
                id: 1,
                name: "Warrior".to_string(),
            },
            active_spec: Some(NamedRef {
                id: 71,
                name: "Arms".to_string(),
            }),
            faction: Some(TypedName {
                kind: "ALLIANCE".to_string(),
                name: "Alliance".to_string(),
            }),
            achievement_points: 12_345,
            average_item_level: 600,
            equipped_item_level: 601,
            last_login_timestamp: Some(1_700_000_000_000),
        })
    }

    async fn item(&self, id: u64) -> Result<Item, BnetError> {
        if id == 404 {
            return Err(BnetError::Api {
                status: http::StatusCode::NOT_FOUND,
                url: format!("https://example.invalid/data/wow/item/{id}"),
                body: r#"{"message":"Item not found"}"#.to_string(),
            });
        }

        Ok(Item {
            id,
            name: "Thunderfury, Blessed Blade of the Windseeker".to_string(),
            quality: TypedName {
                kind: "LEGENDARY".to_string(),
                name: "Legendary".to_string(),
            },
            level: 60,
            required_level: 60,
            item_class: NamedRef {
                id: 2,
                name: "Weapon".to_string(),
            },
            item_subclass: NamedRef {
                id: 7,
                name: "Sword".to_string(),
            },
            inventory_type: Some(TypedName {
                kind: "WEAPON".to_string(),
                name: "One-Hand".to_string(),
            }),
            purchase_price: 0,
            sell_price: 0,
            max_count: 1,
            is_equippable: true,
            is_stackable: false,
        })
    }

    async fn realms(&self) -> Result<Vec<RealmRef>, BnetError> {
        // Deliberately not sorted: the endpoint is responsible for ordering.
        Ok(vec![
            RealmRef {
                id: 1,
                slug: "tichondrius".to_string(),
                name: "Tichondrius".to_string(),
            },
            RealmRef {
                id: 2,
                slug: "area-52".to_string(),
                name: "Area 52".to_string(),
            },
            RealmRef {
                id: 3,
                slug: "stormrage".to_string(),
                name: "Stormrage".to_string(),
            },
            RealmRef {
                id: 4,
                slug: "illidan".to_string(),
                name: "Illidan".to_string(),
            },
            RealmRef {
                id: 5,
                slug: "proudmoore".to_string(),
                name: "Proudmoore".to_string(),
            },
        ])
    }
}

/// Start the real router with a fake Battle.net client and a test logger.
///
/// `TestContext::new` requires port 0, so every test gets a free port and they
/// can run in parallel.
pub fn test_context(test_name: &str) -> TestContext<ApiContext> {
    test_context_with_bnet(test_name, Arc::new(FakeBnet))
}

/// Same as [`test_context`], but lets a test supply its own client.
pub fn test_context_with_bnet(test_name: &str, bnet: Arc<dyn BnetApi>) -> TestContext<ApiContext> {
    let log_context = LogContext::new(
        test_name,
        &ConfigLogging::StderrTerminal {
            level: ConfigLoggingLevel::Warn,
        },
    );
    let log = log_context.log.clone();
    let context = ApiContext::new(bnet);

    TestContext::new(
        api::api_description().expect("route table should build"),
        context,
        &ConfigDropshot::default(),
        Some(log_context),
        log,
    )
}
