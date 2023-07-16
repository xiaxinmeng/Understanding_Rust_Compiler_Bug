rust
pub trait Clone {
    fn clone(&self) -> Self; // as today

    #[sealed]
    fn copy(&self) -> Self { *self }
}
