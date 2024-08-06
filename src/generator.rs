use std::sync::atomic::{AtomicU32, Ordering};

pub struct Generator {
    counter: AtomicU32,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            counter: AtomicU32::new(1),
        }
    }

    pub fn from(value: u32) -> Self {
        Self {
            counter: AtomicU32::new(value),
        }
    }

    pub fn poll(&self) -> u32 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
