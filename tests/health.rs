//! Integration tests for the HTTP API.
//!
//! These tests import the library crate, build the real route table, start a
//! real server on an ephemeral port, and make real HTTP requests. They do not
//! call the handler function directly, so routing, extraction, and JSON
//! serialization are all covered.

mod common;

use dropshot::test_util::read_json;
use http::{Method, StatusCode};
use serde_json::Value;

use common::test_context;

#[tokio::test]
async fn health_reports_ok_and_uptime() {
    let testctx = test_context("health_reports_ok_and_uptime");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(Method::GET, "/health", StatusCode::OK)
        .await
        .expect("GET /health should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(body["status"], "ok");
    assert!(
        body["uptime_seconds"].is_u64(),
        "uptime_seconds should be an unsigned integer, got {body}"
    );

    testctx.teardown().await;
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let testctx = test_context("unknown_route_returns_404");

    let error = testctx
        .client_testctx
        .make_request_error(Method::GET, "/nope", StatusCode::NOT_FOUND)
        .await;
    assert_eq!(error.message, "Not Found");

    testctx.teardown().await;
}
