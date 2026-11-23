//! State shared by every request handler.

use std::time::Instant;

/// Dropshot gives each handler a `&ApiContext`.
///
/// Later this will also hold the Battle.net client. Keeping state in one place
/// means handlers never construct their own HTTP clients or re-read config.
pub struct ApiContext {
    pub started_at: Instant,
}

impl ApiContext {
    pub fn new() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }
}

impl Default for ApiContext {
    fn default() -> Self {
        Self::new()
    }
}
