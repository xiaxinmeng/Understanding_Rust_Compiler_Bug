rust
pub trait FastFloat {
    fn rem_i32(&self, divisor: i32) -> f64;
    fn rem_i64(&self, divisor: i64) -> f64;
    fn rem_i128(&self, divisor: i128) -> f64;
}

impl FastFloat for f64 {
    #[inline]
    fn rem_i32(&self, divisor: i32) -> f64 {
        let x = *self as i32;
        if self.is_sign_positive() {
            // eprintln!("{} % {} = {}", x, divisor, x % divisor);
            (x % divisor) as f64 + self.fract()
        } else {
            // eprintln!("{} % {} = {}", x, divisor, x % divisor);
            (x % divisor + divisor.abs()) as f64 + self.fract()
        }
    }

    #[inline]
    fn rem_i64(&self, divisor: i64) -> f64 {
        let x = *self as i64;
        if self.is_sign_positive() {
            // eprintln!("{} % {} = {}", x, divisor, x % divisor);
            (x % divisor) as f64 + self.fract()
        } else {
            // eprintln!("{} % {} = {}", x, divisor, x % divisor);
            (x % divisor + divisor.abs()) as f64 + self.fract()
        }
    }

    #[inline]
    fn rem_i128(&self, divisor: i128) -> f64 {
        let x = *self as i128;
        if self.is_sign_positive() {
            // eprintln!("{} % {} = {}", x, divisor, x % divisor);
            (x % divisor) as f64 + self.fract()
        } else {
            // eprintln!("{} % {} = {}", x, divisor, x % divisor);
            (x % divisor + divisor.abs()) as f64 + self.fract()
        }
    }
}

#[test]
fn cmp_rem_i32() {
    let x: f64 = 47.76;
    assert_eq!(x.rem_euclid(16.0), x.rem_i32(16));
}

#[test]
fn cmp_rem_i32_neg() {
    let x: f64 = -47.76;
    assert_eq!(x.rem_euclid(16.0), x.rem_i32(16));
}
