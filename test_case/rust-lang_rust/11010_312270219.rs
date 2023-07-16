rust
#[bench]
fn foo_bench(b: &mut Bencher) {
    b.once(|| {
        // Do something slow and expensive
    });
}
