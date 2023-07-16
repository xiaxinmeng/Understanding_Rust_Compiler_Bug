rust
pub trait Trait {
    fn public_fn();
    #[doc(hidden)]
    fn __private_fn() {…}
}

impl Trait for Struct {
    fn public_fn() {…}
    #[doc(hidden)]
    fn __private_fn() {…}
}
