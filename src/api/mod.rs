//! The HTTP API: the route table and its endpoint modules.

pub mod character;
pub mod health;

use dropshot::{ApiDescription, ApiDescriptionRegisterError};

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
    Ok(api)
}
