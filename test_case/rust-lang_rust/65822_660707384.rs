
use std::sync::atomic::{AtomicUsize, Ordering};

const COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}
