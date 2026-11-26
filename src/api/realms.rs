//! `GET /api/realms`, paginated with Dropshot's keyset pagination.

use dropshot::{
    EmptyScanParams, HttpError, HttpResponseOk, PaginationParams, Query, RequestContext,
    ResultsPage, WhichPage, endpoint,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bnet::RealmRef;
use crate::context::ApiContext;

/// Where the next page starts.
///
/// Dropshot serializes this struct to JSON, base64url-encodes it, and returns
/// the result as the opaque `page_token`. Clients never see `last_slug`, which
/// means we can change this struct later without breaking them.
///
/// Rust concept: `Serialize` is needed to mint the token and `Deserialize` to
/// read it back. `JsonSchema` is required by `PaginationParams` even though the
/// token is not a normal query parameter.
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct RealmPage {
    /// Slug of the last realm the client has already seen.
    last_slug: String,
}

/// List every realm in the region, a page at a time.
///
/// There are no scan parameters yet, so `EmptyScanParams` stands in for them.
/// If we later add filters (for example `?locale=` or `?connected=true`), they
/// belong in the scan params so they carry across pages.
#[endpoint {
    method = GET,
    path = "/api/realms",
}]
pub async fn list_realms(
    rqctx: RequestContext<ApiContext>,
    query: Query<PaginationParams<EmptyScanParams, RealmPage>>,
) -> Result<HttpResponseOk<ResultsPage<RealmRef>>, HttpError> {
    let params = query.into_inner();

    // Never trust the client's `limit` directly. `page_limit` clamps it to the
    // server's configured maximum and applies the default when it is absent.
    let limit = rqctx.page_limit(&params)?.get() as usize;

    let mut realms = rqctx.context().bnet.realms().await?;

    // Battle.net does not promise an order, but keyset pagination needs one.
    // The slug is unique and stable, so sort by it.
    realms.sort_by(|left, right| left.slug.cmp(&right.slug));

    let total = realms.len();

    // Keyset pagination: the token names the last item we returned, and the
    // next page starts immediately after it. No offsets, so inserting a realm
    // upstream cannot make a client skip or repeat one.
    let start = match &params.page {
        WhichPage::First(_) => 0,
        WhichPage::Next(RealmPage { last_slug }) => {
            realms.partition_point(|realm| realm.slug.as_str() <= last_slug.as_str())
        }
    };

    let page: Vec<RealmRef> = realms.into_iter().skip(start).take(limit).collect();
    let returned = page.len();

    let mut results = ResultsPage::new(page, &EmptyScanParams {}, |realm, _| RealmPage {
        last_slug: realm.slug.clone(),
    })?;

    // `ResultsPage::new` always emits a token for a non-empty page, even when
    // that page reached the end of the collection. Clearing it here saves the
    // client a pointless extra request that would return zero items.
    if start + returned >= total {
        results.next_page = None;
    }

    Ok(HttpResponseOk(results))
}
