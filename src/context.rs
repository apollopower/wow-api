//! State shared by every request handler.

use std::time::Instant;

use crate::bnet::BnetClient;

/// Dropshot gives each handler a `&ApiContext`.
///
/// The Battle.net client is shared, so its connection pool and token cache are
/// reused across requests. Cloning is cheap; `BnetClient` is built for this.
pub struct ApiContext {
    pub started_at: Instant,
    pub bnet: BnetClient,
}

impl ApiContext {
    pub fn new(bnet: BnetClient) -> Self {
        Self {
            started_at: Instant::now(),
            bnet,
        }
    }
}
