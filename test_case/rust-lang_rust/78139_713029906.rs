rust
        if mem::needs_drop::<T>() {
            unsafe {
                ptr::drop_in_place(...);
            }
        }
