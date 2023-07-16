rust
// what we want to reach
pub fn manual_while() {
    let mut n = 0;
    while n < UPPER {
        test::black_box(n);
        n += STEP;
    }
}
