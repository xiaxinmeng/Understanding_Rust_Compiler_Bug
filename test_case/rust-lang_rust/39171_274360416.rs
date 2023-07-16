
        #[export_name="__umodti3"]
        pub extern "C" fn u128_mod_(a: u128_, b: u128_) -> $cret {
            unsafe {
                let mut r = ::core::mem::zeroed();
                u128_div_mod(a, b, &mut r);
                ($conv)(r)
            }
        }
