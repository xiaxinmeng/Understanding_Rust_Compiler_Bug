rust
            ptr::copy_nonoverlapping(
                namespace.as_ptr(),
                addr.sun_path.as_mut_ptr().offset(1) as *mut u8,
                namespace.len(),
            );
