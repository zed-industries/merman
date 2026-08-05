//! Timing helpers: real `std::time` on native builds, no-ops on wasm.

pub use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

/// No-op stand-in for `std::time::Instant` on wasm builds.
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy)]
pub struct Instant;

#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Self {
        Self
    }

    pub fn elapsed(&self) -> Duration {
        Duration::ZERO
    }
}
