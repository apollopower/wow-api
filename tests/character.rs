//! Integration tests for the character route.

mod common;

use dropshot::test_util::read_json;
use http::{Method, StatusCode};
use serde_json::Value;

use common::test_context;

#[tokio::test]
async fn character_route_returns_bnet_data() {
    let testctx = test_context("character_route_returns_bnet_data");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(
            Method::GET,
            "/api/character/tichondrius/Thrall",
            StatusCode::OK,
        )
        .await
        .expect("GET character should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(body["name"], "Thrall");
    assert_eq!(body["realm"]["slug"], "tichondrius");
    assert_eq!(body["character_class"]["name"], "Warrior");
    assert_eq!(body["level"], 80);

    testctx.teardown().await;
}

#[tokio::test]
async fn character_route_percent_decodes_path_params() {
    let testctx = test_context("character_route_percent_decodes_path_params");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(
            Method::GET,
            "/api/character/tichondrius/Mister%20Fister",
            StatusCode::OK,
        )
        .await
        .expect("GET character should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(body["name"], "Mister Fister");

    testctx.teardown().await;
}

#[tokio::test]
async fn character_not_found_maps_to_404() {
    let testctx = test_context("character_not_found_maps_to_404");

    let error = testctx
        .client_testctx
        .make_request_error(
            Method::GET,
            "/api/character/tichondrius/Missing",
            StatusCode::NOT_FOUND,
        )
        .await;
    assert_eq!(error.message, "Not Found");

    testctx.teardown().await;
}
