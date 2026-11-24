//! Binary entry point.
//!
//! This file is the "composition root": it wires together configuration,
//! logging, the route table, and the server. The route table and handlers live
//! in the library crate so they can be tested.

use dropshot::{ConfigDropshot, ConfigLogging, ConfigLoggingLevel, ServerBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load `.env` for local development if it exists. Existing environment
    // variables take precedence, and a missing file is not an error.
    dotenvy::dotenv().ok();

    let config = wow_api::config::Config::from_env()?;

    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("wow-api")?;

    // `SocketAddr` is `Copy`, so we can save it before moving the rest of the
    // config into the Battle.net client.
    let bind_address = config.bind_address;
    let bnet = wow_api::bnet::BnetClient::new(config)?;
    let context = wow_api::context::ApiContext::new(bnet);

    let api = wow_api::api::api_description()?;

    let server = ServerBuilder::new(api, context, log)
        .config(ConfigDropshot {
            bind_address,
            ..Default::default()
        })
        .start()?;

    println!("listening on http://{}", server.local_addr());
    server.await.map_err(anyhow::Error::msg)?;
    Ok(())
}
