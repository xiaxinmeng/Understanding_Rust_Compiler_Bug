rust
struct BallastType {
    ballast: [u64; 32], // the compiler keeps this, even if unused
    value: usize,
}
