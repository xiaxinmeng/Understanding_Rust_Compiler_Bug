 rust
#[bench(time O(log n), space O(1))]
fn bench_something(b: &mut Bencher) {
    b.iter_with_size(|n| {
        perform_magic!(n)
    });
}
