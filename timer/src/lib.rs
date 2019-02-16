use std::time::{Instant, Duration};
use shared::Link;

pub type SyncTimer = Link<Instant>;

pub trait Timer {
    fn elapsed(&self) -> Duration;
}

impl Timer for SyncTimer {
    fn elapsed(&self) -> Duration {
        let elapsed = self.lock().elapsed();
        elapsed
    }
}