rust
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    static A: AtomicUsize = AtomicUsize::new(42);
    static B: &'static AtomicUsize = &A;
    let x = B;
	A.store(0, Ordering::Relaxed);
    let y = x.load(Ordering::Relaxed);
}
