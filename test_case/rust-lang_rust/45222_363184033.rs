rust
pub struct RangeInclusive<T> {
    // NOTE: not pub
    start: T, // actually, these should probably be ManuallyDrop<T> 
    end: T,   // or union MaybeUninit<T> { value: T, empty: () }
    done: bool,
}

impl RangeInclusive<T> {
    // Expose an API that matches the functionality of the enum type
    #[inline] pub fn new(start: T, end: T) -> Self { ... }
    #[inline] pub fn new_done() -> Self { ... }
    #[inline] pub fn endpoints(&self) -> Option<(&T, &T)> { ... }
    #[inline] pub fn endpoints_mut(&mut self) -> Option<(&mut T, &mut T)> { ... }
    #[inline] pub fn into_endpoints(self) -> Option<(T, T)> { ... }
}
