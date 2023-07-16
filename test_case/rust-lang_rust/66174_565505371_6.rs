rust
    let mut buf: [MaybeUninit<u8>; N] = MaybeUninit::uninit_array();
    let _ = reader.read_into_uninit_exact(&mut buf[..])?;
    unsafe {
        // Safety: `ReadIntoUninit`'s safety guarantees ensure that `buf` has been initialized.
        buf.assume_init()
    }
    