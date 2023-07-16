rust
pub struct MustPin<T: Unpin>(T, Pinned);

impl<T: Unpin> MustPin<T> {
    pub const fn new(t: T) -> Self { ... }
    pub fn get(self: Pin<&Self>) -> *const T { ... }
    pub fn get_mut(self: Pin<&mut Self>) -> *mut T { ... }
}
