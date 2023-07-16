rust
use std::mem;

unsafe fn example<T>() -> (usize, T) {
    (mem::size_of::<T>(), mem::zeroed())
}
