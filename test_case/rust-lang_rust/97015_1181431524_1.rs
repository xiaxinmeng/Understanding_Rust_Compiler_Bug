rust
    pub unsafe trait FromToFFISafe: Sized {
        type Type;
        unsafe fn to_ffi(self) -> Self::Type;
        unsafe fn from_ffi(ffi: Self::Type) -> Self;
    }

    #[repr(C)]
    pub struct FFISafeBorrowBuf<'a> {
        pub buf: *mut u8,
        pub capacity: usize,
        pub filled: usize,
        pub initialized: usize,
        _phantom: PhantomData<&'a mut [u8]>,
    }

    impl<'a> FromToFFISafe for BorrowBuf<'a> {
        type Type = FFISafeBorrowBuf<'a>;
        unsafe fn to_ffi(self) -> Self::Type {
            FFISafeBorrowBuf {
                capacity: self.capacity(),
                filled: self.len(),
                initialized: self.init_len(),
                buf: self.buf_mut().as_mut_ptr(),
            }
        }
        unsafe fn from_ffi(ffi: Self::Type) -> Self {
            let mut retval = Self::from(slice::from_raw_parts_mut(ffi.buf as *mut MaybeUninit<u8>, buf.capacity));
            retval.set_init(ffi.initialized);
            retval.unfilled().advance(ffi.filled);
            retval
        }
    }
    