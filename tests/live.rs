//! Live smoke tests.
//!
//! These are ignored by default because they require real Battle.net
//! credentials and network access. Run them explicitly with:
//!
//! ```text
//! cargo test --test live -- --ignored
//! ```

use wow_api::bnet::{BnetApi, BnetClient};
use wow_api::config::Config;

#[tokio::test]
#[ignore = "requires live Battle.net credentials and network access"]
async fn live_item_lookup() {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("config should load from .env");
    let client = BnetClient::new(config).expect("client should build");

    // Item 19019 is Thunderfury; the ID is stable across game versions.
    let item = client
        .item(19019)
        .await
        .expect("item lookup should succeed");

    assert_eq!(item.id, 19019);
    assert!(!item.name.is_empty());
    println!("{} ({})", item.name, item.quality.name);
}
