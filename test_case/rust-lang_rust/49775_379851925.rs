rust
use std::sync::atomic::*;
use std::sync::atomic::Ordering::SeqCst;

static FOO: AtomicIsize  = AtomicIsize::new(0);

fn main() {
    FOO.load(SeqCst);
}
