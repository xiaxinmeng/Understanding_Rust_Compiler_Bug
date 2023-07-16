 rust
        if buf.len() > dst.len() {
            unsafe { std::intrinsics::abort(); }
        }
