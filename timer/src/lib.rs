use std::time::{Instant, Duration};
use shared::Link;

pub struct SyncTimer(Link<Instant>);

pub trait Timer {
    fn start() -> Self;
    fn elapsed(&self) -> Duration;
}

pub fn run<T: Timer>() -> T {
    T::start()
}

impl Timer for SyncTimer {
    fn start() -> Self {
        SyncTimer(Link::from(Instant::now()))
    }

    fn elapsed(&self) -> Duration {
        self.0.lock().elapsed()
    }
}