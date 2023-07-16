 rust
#[bench]
fn slow(b: &mut test::Bencher) {
    let s: String = iter::repeat("a").take(b.suggested_len).collect();
    b.iter(|| 1 + 1);
    b.bytes = s.len() as u64;
}
