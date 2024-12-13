use std::sync::{Arc, Mutex};

use crate::ExponentialHistogram;

/// An ExponentialHistogram with interior mutability
#[derive(Debug, Clone, Default)]
pub struct SharedExponentialHistogram {
    inner: Arc<Mutex<ExponentialHistogram>>,
}

impl SharedExponentialHistogram {
    /// Observe a value, increasing its bucket's count by 1
    pub fn accumulate(&self, value: f64) {
        self.inner
            .lock()
            .expect("local mutex works")
            .accumulate(value)
    }

    /// Get the current snapshot of the histogram. This gives you an owned clone of the backing histogram
    /// at a point in time, so you can work with it without holding a lock.
    pub fn snapshot(&self) -> ExponentialHistogram {
        self.inner.lock().expect("local mutex works").clone()
    }
}
