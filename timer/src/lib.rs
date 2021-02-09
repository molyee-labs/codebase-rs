use shared::RcCell;
use std::time::{Duration, Instant};

pub struct SyncTimer(RcCell<Instant>);

pub trait Timer {
    fn start() -> Self;
    fn elapsed(&self) -> Duration;
}

pub fn run<T: Timer>() -> T {
    T::start()
}

impl Timer for SyncTimer {
    fn start() -> Self {
        Self(RcCell::new(Instant::now()))
    }

    fn elapsed(&self) -> Duration {
        self.0.get().elapsed()
    }
}
