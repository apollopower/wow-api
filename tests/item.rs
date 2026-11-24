//! Integration tests for the item route.

mod common;

use dropshot::test_util::read_json;
use http::{Method, StatusCode};
use serde_json::Value;

use common::test_context;

#[tokio::test]
async fn item_route_returns_bnet_data() {
    let testctx = test_context("item_route_returns_bnet_data");

    let mut response = testctx
        .client_testctx
        .make_request_no_body(Method::GET, "/api/item/19019", StatusCode::OK)
        .await
        .expect("GET item should succeed");

    let body: Value = read_json(&mut response).await;
    assert_eq!(body["id"].as_u64(), Some(19019));
    assert_eq!(body["name"], "Thunderfury, Blessed Blade of the Windseeker");
    assert_eq!(body["quality"]["name"], "Legendary");
    assert_eq!(body["item_class"]["name"], "Weapon");

    testctx.teardown().await;
}

#[tokio::test]
async fn item_not_found_maps_to_404() {
    let testctx = test_context("item_not_found_maps_to_404");

    let error = testctx
        .client_testctx
        .make_request_error(Method::GET, "/api/item/404", StatusCode::NOT_FOUND)
        .await;
    assert_eq!(error.message, "Not Found");

    testctx.teardown().await;
}
