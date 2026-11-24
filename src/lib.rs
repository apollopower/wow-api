//! Library crate for `wow-api`.
//!
//! The binary in `src/main.rs` is intentionally tiny. Everything that can be
//! tested lives here, so integration tests in `tests/` can import this crate
//! and exercise the real router.

pub mod api;
pub mod bnet;
pub mod config;
pub mod context;
