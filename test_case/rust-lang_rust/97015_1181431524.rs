rust
    let mut buf = BorrowBuf::from(&mut MaybeUninit::uninit_array::<256>());
    reader.read_buf(buf.unfilled())?;
    let len = unsafe { process_in_place_ffi(buf.buf_mut().as_mut_ptr() as *mut u8, buf.capacity(), buf.len()) };
    buf.clear();
    unsafe { buf.unfilled().advance(len); }
    println!("{:?}", buf.filled());
    