
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    AtomicU64::new(19).fetch_sub(6, Ordering::SeqCst);
}
