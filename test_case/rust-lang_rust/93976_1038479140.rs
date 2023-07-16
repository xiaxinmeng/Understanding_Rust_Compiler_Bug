rust
        let mut s: OsString = ...
        s.push(unsafe {
            std::str::from_utf8_unchecked(slice::from_ref(&(MAIN_SEPARATOR as u8)))
        });
