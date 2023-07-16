
// libcore
pub fn assert_receiver_is_clone<T: Clone + ?Sized>() {}

union U { a: u8, b: u16 }

impl ::std::clone::Clone for U {
    #[inline]
    fn clone(&self) -> U {
        ::std::clone::assert_receiver_is_clone::<u8>();
        ::std::clone::assert_receiver_is_clone::<u16>();
        *self
    }
}
