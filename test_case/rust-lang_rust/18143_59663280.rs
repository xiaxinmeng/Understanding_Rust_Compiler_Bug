 rust
    let mut end = data.as_ptr().offset(data.len() as int - 1);
    let mut cur = end;
    loop {
        if *cur != b'>' {
            // asm!("");
            cur = std::intrinsics::offset(cur, -1);
            continue;
        }
