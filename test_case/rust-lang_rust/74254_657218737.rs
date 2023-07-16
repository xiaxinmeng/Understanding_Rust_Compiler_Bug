rust
/// A trait implemented by all arrays which are either empty or contain a type implementing `Default`.
#[unstable(feature = "array_default_internals", reason = "implementation detail", issue = "none")]
#[marker]
pub trait ArrayDefault {}

#[unstable(feature = "array_default_internals", reason = "implementation detail", issue = "none")]
impl<T> ArrayDefault for [T; 0] {}

#[unstable(feature = "array_default_internals", reason = "implementation detail", issue = "none")]
impl<T: Default, const N: usize> ArrayDefault for [T; N] {}

trait DefaultHack {
    fn default_hack() -> Self;
}

impl<T> DefaultHack for T {
    default fn default_hack() -> Self {
        unreachable!();
    }
}

impl<T: Default> DefaultHack for T {
    fn default_hack() -> Self {
        Default::default()
    }
}

#[stable(since = "1.4.0", feature = "array_default")]
impl<T, const N: usize> Default for [T; N]
where
    [T; N]: ArrayDefault
{
    // ...
}
