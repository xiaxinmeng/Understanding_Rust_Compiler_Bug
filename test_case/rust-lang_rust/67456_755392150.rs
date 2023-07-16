Rust
#![feature(const_ptr_is_null)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_raw_ptr_deref)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_panic)]


mod slice {
    use core::mem;
    use core::ptr;
    
    pub(crate) const unsafe fn is_aligned_and_not_null<T>(ptr: *const T) -> bool {
        !ptr.is_null() && ptr as usize % mem::align_of::<T>() == 0
    }

    pub const unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T] {
        debug_assert!(
            is_aligned_and_not_null(data),
            "attempt to create unaligned or null slice"
        );
        debug_assert!(
            mem::size_of::<T>().saturating_mul(len) <= isize::MAX as usize,
            "attempt to create slice covering at least half the address space"
        );
        // SAFETY: the caller must uphold the safety contract for `from_raw_parts`.
        unsafe { &*ptr::slice_from_raw_parts(data, len) }
    }
    
    pub const fn from_ref<T>(reference: &T) -> &[T] {
        unsafe {
            from_raw_parts(reference, 1)
        }
    }
}

const X: &[usize] = slice::from_ref(&1);

fn main() {
    dbg!(X);
}
