//! Integration tests for the character route.

mod common;

use dropshot::test_util::read_json;
use http::{Method, StatusCode};
use serde_json::Value;

use common::test_context;

#[tokio::test]
async fn character_route_echoes_path_params() {
    let testctx = test_context("character_route_echoes_path_params");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(
            Method::GET,
            "/api/character/tichondrius/Thrall",
            StatusCode::OK,
        )
        .await
        .expect("GET character stub should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(body["realm"], "tichondrius");
    assert_eq!(body["name"], "Thrall");
    assert!(
        body["note"]
            .as_str()
            .is_some_and(|note| note.contains("stub")),
        "expected a stub note, got {body}"
    );

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
        .expect("GET character stub should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(body["name"], "Mister Fister");

    testctx.teardown().await;
}
