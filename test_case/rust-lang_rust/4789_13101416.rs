
pub trait Round {
    pure fn round(&self, mode: RoundMode) -> Self;

    pure fn floor(&self) -> Self;
    pure fn ceil(&self) -> Self;
    pure fn fract(&self) -> Self;
}

pub enum RoundMode {
    RoundDown,
    RoundUp,
    RoundToZero,
    RoundFromZero
}
