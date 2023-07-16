rust
impl<T, const N: usize> TryFrom<Box<[T]>> for Box<[T; N]>
    where
        [T; N]: LengthAtMost32,
{
    type Error = Box<[T]>;
    // …
}