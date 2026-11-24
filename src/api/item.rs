//! `GET /api/item/{id}`.

use dropshot::{HttpError, HttpResponseOk, Path, RequestContext, endpoint};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::bnet::Item;
use crate::context::ApiContext;

/// Path parameters for the item route.
///
/// `id` is a `u64`, so Dropshot will reject a non-numeric path segment before
/// the handler runs.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ItemPath {
    pub id: u64,
}

#[endpoint {
    method = GET,
    path = "/api/item/{id}",
}]
pub async fn get_item(
    rqctx: RequestContext<ApiContext>,
    path: Path<ItemPath>,
) -> Result<HttpResponseOk<Item>, HttpError> {
    let path = path.into_inner();
    let item = rqctx.context().bnet.item(path.id).await?;
    Ok(HttpResponseOk(item))
}
