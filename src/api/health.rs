//! `GET /health`.

use dropshot::{HttpError, HttpResponseOk, RequestContext, endpoint};
use schemars::JsonSchema;
use serde::Serialize;

use crate::context::ApiContext;

#[derive(Debug, Serialize, JsonSchema)]
pub struct Health {
    pub status: String,
    pub uptime_seconds: u64,
}

#[endpoint {
    method = GET,
    path = "/health",
}]
pub async fn health(
    rqctx: RequestContext<ApiContext>,
) -> Result<HttpResponseOk<Health>, HttpError> {
    let context = rqctx.context();

    Ok(HttpResponseOk(Health {
        status: "ok".to_string(),
        uptime_seconds: context.started_at.elapsed().as_secs(),
    }))
}
