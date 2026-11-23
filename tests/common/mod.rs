//! Shared helpers for integration tests.
//!
//! `tests/common/mod.rs` is not compiled as its own test crate. Each test file
//! opts in with `mod common;`, which is the standard Cargo pattern for shared
//! integration-test setup.

use dropshot::test_util::{LogContext, TestContext};
use dropshot::{ConfigDropshot, ConfigLogging, ConfigLoggingLevel};

use wow_api::api;
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

    TestContext::new(
        api::api_description().expect("route table should build"),
        ApiContext::new(),
        &ConfigDropshot::default(),
        Some(log_context),
        log,
    )
}
