 rust
pub trait Trigonometric {
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self, other: Self) -> Self;
}

pub trait Exponential {
    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn expm1(&self) -> Self;
    fn log(&self) -> Self;
    fn log2(&self) -> Self;
    fn log10(&self) -> Self;

    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn tanh(&self) -> Self;
}

pub trait Transcendental: Trigonometric
                        + Exponential {}
