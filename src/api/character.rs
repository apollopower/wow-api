//! `GET /api/character/{realm}/{name}` — currently a stub.

use dropshot::{HttpError, HttpResponseOk, Path, RequestContext, endpoint};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

/// Temporary response shape until the Battle.net client exists.
#[derive(Debug, Serialize, JsonSchema)]
pub struct CharacterStub {
    pub realm: String,
    pub name: String,
    pub note: String,
}

#[endpoint {
    method = GET,
    path = "/api/character/{realm}/{name}",
}]
pub async fn get_character(
    _rqctx: RequestContext<ApiContext>,
    path: Path<CharacterPath>,
) -> Result<HttpResponseOk<CharacterStub>, HttpError> {
    // `Path<T>` is a wrapper. `into_inner()` takes the `CharacterPath` value
    // out of it so we can move its `String` fields into the response.
    let path = path.into_inner();

    Ok(HttpResponseOk(CharacterStub {
        realm: path.realm,
        name: path.name,
        note: "stub: this will call Battle.net later".to_string(),
    }))
}
