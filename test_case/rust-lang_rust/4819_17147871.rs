 rust
pub trait Algebraic {
    fn pow(&self, n: Self) -> Self;
    fn sqrt(&self) -> Self;
    fn rsqrt(&self) -> Self;
    fn cbrt(&self) -> Self;
    fn hypot(&self, other: Self) -> Self;
}

pub trait Exponential {
    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn expm1(&self) -> Self;
    fn log(&self) -> Self;
    fn log2(&self) -> Self;
    fn log10(&self) -> Self;
}

pub trait Triganomic {
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self, other: Self) -> Self;
    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn tanh(&self) -> Self;
}

pub trait Transcendental: Triganomic
                        + Exponential {}

pub trait Real: Signed
              + Fractional
              + Algebraic
              + Transcendental {
    // Not sure where to put these
    fn pi() -> Self;
    fn two_pi() -> Self;
    fn frac_pi_2() -> Self;
    fn frac_pi_3() -> Self;
    fn frac_pi_4() -> Self;
    fn frac_pi_6() -> Self;
    fn frac_pi_8() -> Self;
    fn frac_1_pi() -> Self;
    fn frac_2_pi() -> Self;
    fn frac_2_sqrtpi() -> Self;
    fn sqrt2() -> Self;
    fn frac_1_sqrt2() -> Self;
    fn e() -> Self;
    fn log2_e() -> Self;
    fn log10_e() -> Self;
    fn log_2() -> Self;
    fn log_10() -> Self;

    fn to_degrees(&self) -> Self;
    fn to_radians(&self) -> Self;
}
