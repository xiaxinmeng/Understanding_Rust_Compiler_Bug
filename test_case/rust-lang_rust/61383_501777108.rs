rust
    fn default_impl()->Self{
        unsafe {
            use std::mem::MaybeUninit;
            let mut res = MaybeUninit::<Self>::uninit();
            let res_mut_ptr = res.as_mut_ptr() as *mut T;
            for i in 0 .. N {
                *res_mut_ptr.add(i) = T::default();
            }
            res.assume_init()
        }
    }
