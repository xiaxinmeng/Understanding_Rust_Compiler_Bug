
pub trait Round {
    pure fn ceil(&self) -> Self;
    pure fn floor(&self) -> Self;
    pure fn round(&self) -> Self;
    pure fn trunc(&self) -> Self;
}
