 rust
    let mut buf2 = Vec::with_capacity(268435455);
    unsafe {
        buf2.set_len(268435455);
        buf2.copy_from_slice(&buf1);
    }
