 rust
const BYTES: u64 = 100_000;

#[bench]
fn push_str_one_byte(b: &mut Bencher) {
    b.bytes = BYTES;
    b.iter(|| {
        let mut s = String::new();
        for _ in range(0, BYTES) {
            s.push_str("a")
        }
        black_box(s);
    });
}
