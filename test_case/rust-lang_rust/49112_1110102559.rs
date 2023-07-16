rust
macro_rules! arr {
    ($item:expr; $n:expr) => {
        {
            let mut array_uninit: [_; $n] =
                [::core::mem::MaybeUninit::uninit(); $n];
            for x in &mut array_uninit {
                let item = $item;
                #[allow(unused_unsafe)]
                unsafe { ::core::ptr::write(x.as_mut_ptr(), item) };
            }
            #[allow(unused_unsafe)]
            array_uninit.map(|mu| unsafe { mu.assume_init() })
        }
    }
}
