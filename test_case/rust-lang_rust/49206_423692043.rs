rust
fn foo() -> usize {
    let counter = &AtomicUsize::new(0);
    counter.fetch_add(1, Ordering::SeqCst)
}
