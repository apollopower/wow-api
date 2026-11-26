//! Integration tests for the paginated realm index.

mod common;

use dropshot::test_util::read_json;
use http::{Method, StatusCode};
use serde_json::Value;

use common::test_context;

/// The realm slugs on a page, in the order the server returned them.
fn slugs(body: &Value) -> Vec<String> {
    body["items"]
        .as_array()
        .expect("items should be an array")
        .iter()
        .map(|item| {
            item["slug"]
                .as_str()
                .expect("each realm should have a slug")
                .to_string()
        })
        .collect()
}

/// Page tokens are base64url, so the only character that needs encoding in a
/// query string is the `=` padding.
fn query_token(body: &Value) -> String {
    body["next_page"]
        .as_str()
        .expect("page should have a next_page token")
        .replace('=', "%3D")
}

#[tokio::test]
async fn realms_are_sorted_and_limited() {
    let testctx = test_context("realms_are_sorted_and_limited");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(Method::GET, "/api/realms?limit=2", StatusCode::OK)
        .await
        .expect("GET realms should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(slugs(&body), ["area-52", "illidan"]);
    assert!(body["next_page"].is_string());

    testctx.teardown().await;
}

#[tokio::test]
async fn realms_paginate_to_the_end() {
    let testctx = test_context("realms_paginate_to_the_end");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(Method::GET, "/api/realms?limit=2", StatusCode::OK)
        .await
        .expect("first page should succeed");
    let first: Value = read_json(&mut response).await;
    assert_eq!(slugs(&first), ["area-52", "illidan"]);

    let url = format!("/api/realms?limit=2&page_token={}", query_token(&first));
    let mut response = testctx
        .client_testctx
        .make_request_no_body(Method::GET, &url, StatusCode::OK)
        .await
        .expect("second page should succeed");
    let second: Value = read_json(&mut response).await;
    assert_eq!(slugs(&second), ["proudmoore", "stormrage"]);

    let url = format!("/api/realms?limit=2&page_token={}", query_token(&second));
    let mut response = testctx
        .client_testctx
        .make_request_no_body(Method::GET, &url, StatusCode::OK)
        .await
        .expect("third page should succeed");
    let third: Value = read_json(&mut response).await;
    assert_eq!(slugs(&third), ["tichondrius"]);

    // The final page must not hand out another token.
    assert!(third["next_page"].is_null());

    testctx.teardown().await;
}

#[tokio::test]
async fn realms_without_limit_returns_everything() {
    let testctx = test_context("realms_without_limit_returns_everything");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(Method::GET, "/api/realms", StatusCode::OK)
        .await
        .expect("GET realms should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(
        slugs(&body),
        [
            "area-52",
            "illidan",
            "proudmoore",
            "stormrage",
            "tichondrius"
        ]
    );
    assert!(body["next_page"].is_null());

    testctx.teardown().await;
}

#[tokio::test]
async fn realms_reject_invalid_page_token() {
    let testctx = test_context("realms_reject_invalid_page_token");

    let error = testctx
        .client_testctx
        .make_request_error(
            Method::GET,
            "/api/realms?page_token=not-a-real-token",
            StatusCode::BAD_REQUEST,
        )
        .await;
    assert!(!error.message.is_empty());

    testctx.teardown().await;
}

#[tokio::test]
async fn realms_reject_zero_limit() {
    let testctx = test_context("realms_reject_zero_limit");

    // `limit` is deserialized as `NonZeroU32`, so zero never reaches the
    // handler.
    testctx
        .client_testctx
        .make_request_error(Method::GET, "/api/realms?limit=0", StatusCode::BAD_REQUEST)
        .await;

    testctx.teardown().await;
}
