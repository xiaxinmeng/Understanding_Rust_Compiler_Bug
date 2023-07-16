rust
let mut buf = Vec::with_capacity(1024);
let read = file.read(&mut buf)?; // specialize if `Vec` is passed.
let data = buf[..read];
