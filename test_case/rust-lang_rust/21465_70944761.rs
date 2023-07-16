 rust
#[inline(never)]
fn extend7<T, I: Iterator<Item=T>+ExactSizeIterator>(vec: &mut Vec<T>, iter: I) {
    struct PanicGuard<'a, T: 'a> {
        vec: &'a mut Vec<T>,
        ptr: *const *mut T,
    }

    #[unsafe_destructor]
    impl<'a, T> Drop for PanicGuard<'a, T> {
        fn drop(&mut self) {
            unsafe {
                let diff = *self.ptr as usize - self.vec.as_ptr() as usize;
                let size = mem::size_of::<T>();
                let len = diff / if size == 0 { 1 } else { size };
                self.vec.set_len(len);
            }
        }
    }

    let mut iter = iter;
    let len = iter.len();
    vec.reserve(len);

    unsafe {
        {
            let mut ptr = if mem::size_of::<T>() == 0 {
                (vec.as_mut_ptr() as usize + vec.len()) as *mut T
            } else {
                vec.as_mut_ptr().offset(vec.len() as isize)
            };
            let guard = PanicGuard { vec: vec, ptr: &ptr };
            for el in iter {
               ptr::write(ptr, el);
               ptr = if mem::size_of::<T>() == 0 {
                   (ptr as usize + 1) as *mut T
               } else {
                   ptr.offset(1)
               };
            }
            mem::forget(guard);
        }
        vec.set_len(len);
    }
}
