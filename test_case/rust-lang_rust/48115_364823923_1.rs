rust
pub struct Flatten<I, U> {
    iter: I,
    frontiter: Option<U>,
    backiter: Option<U>,
}
