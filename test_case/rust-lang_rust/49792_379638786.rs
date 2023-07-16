rust
#[marker] unsafe trait InplaceReinterpretAs<T> {}
unsafe impl<T> InplaceReinterpretAs<T> for T {}
unsafe impl InplaceReinterpretAs<[u8; 4]> for u32 {}
unsafe impl InplaceReinterpretAs<i32> for u32 {}
unsafe impl InplaceReinterpretAs<u32> for i32 {}
unsafe impl<T, U> InplaceReinterpretAs<*const U> for *const T {}
unsafe impl<T, U> InplaceReinterpretAs<*mut U> for *mut T {}
unsafe impl InplaceReinterpretAs<u16x8> for u32x4 {}
unsafe impl InplaceReinterpretAs<u32x4> for u16x8 {}

#[marker] unsafe trait ReinterpretAs<T> {
    // Because it's a marker trait, these cannot be overridden,
    // and thus their behaviour is always predicatable
    fn reinterpret(self) -> T {
        unsafe {
            let r = ptr::read_unaligned(&self as *const Self as *const T);
            mem::forget(self);
            r
        }
    }
    unsafe fn reinterpret_unchecked(x: T) -> Self {
        let r = ptr::read_unaligned(&x as *const T as *const Self);
        mem::forget(x);
        r
    }
}
unsafe impl<T, U> ReinterpretAs<U> for T where T: InplaceReinterpretAs<U> {}
unsafe impl<'a, T, U> ReinterpretAs<&'a U> for &'a T where T: InplaceReinterpretAs<U> {}
unsafe impl<'a, T, U> ReinterpretAs<&'a mut U> for &'a mut T where T: InplaceReinterpretAs<U> {}
unsafe impl ReinterpretAs<u32> for [u8;4] {} // not ok in-place, but fine as memcpy
