 rust
            unsafe {
                cast::transmute(Slice {
                    data: ptr::mut_offset(p, start as int) as *T,
                    len: (end - start) * sys::nonzero_size_of::<T>()
                })
            }
