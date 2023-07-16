 rust
#[bench]
fn slow(b: &mut test::Bencher) {
    let size = if b.is_benching { 5 * 1024 * 1024 } else { 1 };
    let s: String = iter::repeat("a").take(size).collect();
    b.iter(|| 1 + 1);
    b.bytes = s.len() as u64;
}
