rust
use std::{panic, thread, time::Duration};

fn main() {
    let _ = panic::catch_unwind(|| {
        thread::park_timeout(Duration::from_millis(10));
    }); // original panic
    let _ = panic::catch_unwind(|| {
        thread::park_timeout(Duration::from_millis(10));
    }); // “inconsistent park_timeout state”
    let _ = panic::catch_unwind(|| {
        thread::park_timeout(Duration::from_millis(10));
    }); // “PoisonError” (from `thread.inner.lock`)
}
