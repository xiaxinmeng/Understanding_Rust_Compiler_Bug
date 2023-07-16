rust
#[bench]
fn tiny_xorshift(b: &mut Bencher) {
    let mut y = 2463534242u32;
    b.bytes = 4;
    b.iter(|| {
        y ^= y << 13;
        y ^= y >> 17;
        y ^= y << 5;
        black_box(y);
    });
}
