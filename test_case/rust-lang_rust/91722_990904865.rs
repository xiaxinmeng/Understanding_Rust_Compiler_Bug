rust
    loop {
        if  buf.capacity() - buf.len() < 4 {
            buf.reserve(32); // buf is full, need more space
        }
