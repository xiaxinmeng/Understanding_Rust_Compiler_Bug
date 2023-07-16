rust
            let ptr = "abc".as_ptr();
            let len = 0 - 0;
            super::from_utf8_unchecked(slice::from_raw_parts(ptr, len))
