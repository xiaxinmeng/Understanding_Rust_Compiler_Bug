
/// Constructs a new `Pin<Box<T>>`. If `T` does not implement `PinNotNeeded`, then
/// `x` will be pinned in memory and unable to be moved.
pub fn pin(x: T) -> Pin<Box<T>> {
    (box x).into()
}
