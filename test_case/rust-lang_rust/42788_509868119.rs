rust
// std::mem
impl<T> MaybeUninit<T> {
    pub fn init_all(uninit: &mut [MaybeUninit<T>], val: T) -> &mut [T]  where T: Copy { // `T` could even be `Clone` since this method is panic-safe except for leaks
        ... // write `val` to all elements
        Self::as_init_all(uninit)
    }

    pub fn init_copy(uninit: &mut [MaybeUninit<T>], data: &[T]) -> &mut [T] where T: Copy { // `T` could also be `Clone`
        assert_eq!(uninit.len(), data.len());
        unsafe {
            Self::as_first_ptr(uninit).copy(data, uninit.len());
        }
        Self::as_init_all(uninit)
    }

    pub unsafe fn as_init_all(uninit: &mut [MaybeUninit<T>] -> &mut [T] { 
        ... // assume-initialized intrinsic
        mem::transmute(uninit)
    }
}

// std::io
pub trait Read {
    // ... existing methods

    fn read_uninit(&mut self, uninit: &mut [MaybeUninit<u8>]) -> io::Result<()> {
        self.read(MaybeUninit::init_all(uninit, 0))
    }
}
