rust
fn add_fetch(a: &AtomicUsize, b: usize, order: Ordering) -> usize {
    a.fetch_add(b, order) + b
}
