rs
impl<T> AtomicPtr<T> {
    // These are useful, but is `*mut T` the right argument type? (question 1)
    pub fn fetch_or(&self, a: *mut T) -> *mut T;
    pub fn fetch_and(&self, a:  *mut T) -> *mut T;

    // (This one is clearly cursed, but is actually useful for doing a bit-flip
    // on one of the low bits on an pointer, but perhaps it will just tempt
    // people to make xor lists 😢).
    pub fn fetch_xor(&self, a:  *mut T) -> *mut T;

    // Are these coherent? (question 2)
    pub fn fetch_nand(&self, a:  *mut T) -> *mut T;
    pub fn fetch_min(&self, a:  *mut T) -> *mut T;
    pub fn fetch_max(&self, a:  *mut T) -> *mut T;

    // For these, what are the units of `a`? (question 3)
    pub fn fetch_wrapping_add(&self, a: usize) -> *mut T;
    pub fn fetch_wrapping_sub(&self, a: usize) -> *mut T;
    pub fn fetch_wrapping_offset(&self, a: isize) -> *mut T;
}
