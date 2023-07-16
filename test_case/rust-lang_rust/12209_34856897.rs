 rust
let n = BigUint::from_uint(1) << 1000;
bh.iter(|| {
    let mut m = n.clone();
    for _ in range(0, 100) {
        m = m >> 1;
    }
})
