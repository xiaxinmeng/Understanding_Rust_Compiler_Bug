rust
    let ptr = buf.as_mut_ptr().offset(buf.len() as isize);
    let size = buf.capacity() - buf.len();
    let actual = os_read(ptr, size);
    buf.set_len(buf.len() + actual);
