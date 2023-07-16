rust
impl<T> Option<T> {
    pub const fn as_pin_ref(self: Pin<&Self>) -> Option<Pin<&T>>;
    pub const fn as_pin_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>>;

    pub const unsafe fn unwrap_unchecked(self) -> T;
}

impl<T: Copy> Option<&mut T> {
    pub const fn copied(self) -> Option<T>
}
