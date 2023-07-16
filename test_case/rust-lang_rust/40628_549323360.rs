rust
use std::sync::atomic::{AtomicU32, Ordering};

static X: AtomicU32 = AtomicU32::new(42);

fn foo() {
    X.store(13, Ordering::SeqCst);
    panic!("oh no, a panic!");
    X.store(42, Ordering::SeqCst);
}

fn bar() {
    assert_eq!(X.load(Ordering::SeqCst), 42, "logic error");
}

fn main() {
    let _ = std::panic::catch_unwind(|| foo());
    bar();
}
