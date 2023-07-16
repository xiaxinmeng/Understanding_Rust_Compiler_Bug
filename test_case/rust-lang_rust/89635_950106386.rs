rust
pub trait Covariant<'a> {
    type Target;
    fn unwrap<'c>(target: <Self as Covariant<'c>>::Target) -> Self where Self: Covariant<'c>;
}
