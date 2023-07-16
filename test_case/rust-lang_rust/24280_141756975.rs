 rust
        unsafe {
            let mut data = <whatever we drop>;
            let raw_vec = alloc::raw_vec::RawVec::from_raw_parts(
                                data.as_mut_ptr(), data.capacity());
            mem::forget(data);
            mem::drop(raw_vec);
        }
