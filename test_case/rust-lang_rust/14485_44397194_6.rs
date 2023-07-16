 rust
        if buf.len() > dst.len() {
            unsafe { libc::exit(1); }
        }
