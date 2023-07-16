rust
impl<T> From<&Option<T>> for Option<&T> // equivalent to as_ref()
impl<T> From<&mut Option<T>> for Option<&mut T> // equivalent to as_mut()
