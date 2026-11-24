//! `GET /api/character/{realm}/{name}`.

use dropshot::{HttpError, HttpResponseOk, Path, RequestContext, endpoint};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::bnet::CharacterSummary;
use crate::context::ApiContext;

/// Path parameters for the character route.
///
/// The field names must match the `{realm}` and `{name}` placeholders in the
/// `#[endpoint]` path. Dropshot percent-decodes each segment and deserializes
/// it into this struct.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CharacterPath {
    pub realm: String,
    pub name: String,
}

#[endpoint {
    method = GET,
    path = "/api/character/{realm}/{name}",
}]
pub async fn get_character(
    rqctx: RequestContext<ApiContext>,
    path: Path<CharacterPath>,
) -> Result<HttpResponseOk<CharacterSummary>, HttpError> {
    let path = path.into_inner();

    // `?` uses `From<BnetError> for HttpError`, defined in `api/mod.rs`.
    let character = rqctx
        .context()
        .bnet
        .character(&path.realm, &path.name)
        .await?;

    Ok(HttpResponseOk(character))
}
