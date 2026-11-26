//! `GET /api/realms`, paginated with Dropshot's keyset pagination.

use dropshot::{
    EmptyScanParams, HttpError, HttpResponseOk, PaginationParams, Query, RequestContext,
    ResultsPage, WhichPage, endpoint,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bnet::RealmRef;
use crate::context::ApiContext;

/// Payload of the opaque `page_token`. Clients never see these fields.
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
pub struct RealmPage {
    last_slug: String,
}

/// List every realm in the region, a page at a time.
#[endpoint {
    method = GET,
    path = "/api/realms",
}]
pub async fn list_realms(
    rqctx: RequestContext<ApiContext>,
    query: Query<PaginationParams<EmptyScanParams, RealmPage>>,
) -> Result<HttpResponseOk<ResultsPage<RealmRef>>, HttpError> {
    let params = query.into_inner();

    // Clamps the client's limit to the configured maximum and applies the default.
    let limit = rqctx.page_limit(&params)?.get() as usize;

    let mut realms = rqctx.context().bnet.realms().await?;

    // Battle.net does not guarantee an order, and keyset pagination needs one.
    realms.sort_by(|left, right| left.slug.cmp(&right.slug));

    let total = realms.len();
    let start = match &params.page {
        WhichPage::First(_) => 0,
        WhichPage::Next(RealmPage { last_slug }) => {
            realms.partition_point(|realm| realm.slug.as_str() <= last_slug.as_str())
        }
    };

    let page: Vec<RealmRef> = realms.into_iter().skip(start).take(limit).collect();
    let mut results = ResultsPage::new(page, &EmptyScanParams {}, |realm, _| RealmPage {
        last_slug: realm.slug.clone(),
    })?;

    // `ResultsPage::new` emits a token even on the last page; drop it so clients
    // do not make an extra empty request.
    if start + results.items.len() >= total {
        results.next_page = None;
    }

    Ok(HttpResponseOk(results))
}
