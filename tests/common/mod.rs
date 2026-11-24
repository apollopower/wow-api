//! Shared helpers for integration tests.
//!
//! `tests/common/mod.rs` is not compiled as its own test crate. Each test file
//! opts in with `mod common;`, which is the standard Cargo pattern for shared
//! integration-test setup.

use dropshot::test_util::{LogContext, TestContext};
use dropshot::{ConfigDropshot, ConfigLogging, ConfigLoggingLevel};

use wow_api::api;
use wow_api::bnet::BnetClient;
use wow_api::config::{Config, Region};
use wow_api::context::ApiContext;

/// Start the real router with a fresh context and a test logger.
///
/// `TestContext::new` requires port 0, so every test gets a free port and they
/// can run in parallel.
pub fn test_context(test_name: &str) -> TestContext<ApiContext> {
    let log_context = LogContext::new(
        test_name,
        &ConfigLogging::StderrTerminal {
            level: ConfigLoggingLevel::Warn,
        },
    );
    let log = log_context.log.clone();

    // Dummy credentials are fine: building the client does not make any
    // network requests. Tests that exercise Battle.net will use a fake.
    let config = Config {
        bind_address: "127.0.0.1:0".parse().expect("valid test address"),
        bnet_client_id: "test-client-id".to_string(),
        bnet_client_secret: "test-client-secret".to_string(),
        region: Region::Us,
        locale: "en_US".to_string(),
    };
    let bnet = BnetClient::new(config).expect("test client should build");
    let context = ApiContext::new(bnet);

    TestContext::new(
        api::api_description().expect("route table should build"),
        context,
        &ConfigDropshot::default(),
        Some(log_context),
        log,
    )
}
