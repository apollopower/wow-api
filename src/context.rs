//! State shared by every request handler.

use std::sync::Arc;
use std::time::Instant;

use crate::bnet::BnetApi;

/// Dropshot gives each handler a `&ApiContext`.
///
/// `bnet` is a trait object, so production can use the real Battle.net client
/// while tests use an in-memory fake.
pub struct ApiContext {
    pub started_at: Instant,
    pub bnet: Arc<dyn BnetApi>,
}

impl ApiContext {
    pub fn new(bnet: Arc<dyn BnetApi>) -> Self {
        Self {
            started_at: Instant::now(),
            bnet,
        }
    }
}
