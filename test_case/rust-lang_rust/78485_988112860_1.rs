rust
let mut buf = Vec::with_capacity(1024);
let read = file.read_to_capacity(&mut buf)?; // similar to `read_buf` above, but takes `Vec`
let data = buf[..read];
