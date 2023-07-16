rust
impl<T, U> PartialEq<Box<T>> for Box<U>
    where T: ?Sized, U: ?Sized + PartialEq<T>
{
    // …
}
