//! The HTTP API: the route table and its endpoint modules.

pub mod character;
pub mod health;
pub mod item;

use dropshot::{ApiDescription, ApiDescriptionRegisterError, HttpError};

use crate::bnet::BnetError;
use crate::context::ApiContext;

/// Build the complete route table.
///
/// This is deliberately separate from `main.rs`: tests can call this function
/// and start a server on an ephemeral port without reading environment
/// variables or binding to a fixed address.
pub fn api_description() -> Result<ApiDescription<ApiContext>, ApiDescriptionRegisterError> {
    let mut api = ApiDescription::new();
    api.register(health::health)?;
    api.register(character::get_character)?;
    api.register(item::get_item)?;
    Ok(api)
}

/// Map Battle.net failures to HTTP responses.
///
/// The `?` operator in handlers uses this conversion. We preserve a 404 from
/// Battle.net, turn network failures into 503, and treat everything else as an
/// internal error.
impl From<BnetError> for HttpError {
    fn from(err: BnetError) -> Self {
        match &err {
            BnetError::Api { status, .. } if *status == reqwest::StatusCode::NOT_FOUND => {
                HttpError::for_not_found(None, err.to_string())
            }
            BnetError::Http(_) => {
                HttpError::for_unavail(Some("UpstreamUnavailable".to_string()), err.to_string())
            }
            _ => HttpError::for_internal_error(err.to_string()),
        }
    }
}
