 rust
struct BigFloat;

impl Add for (f64, BigFloat) {
    type Output = BigFloat;

    fn add(self) -> BigFloat {
        BigFloat
    }
}

impl Add for (BigFloat, f64) {
    type Output = BigFloat;

    fn add(self) -> BigFloat {
        BigFloat
    }
}
