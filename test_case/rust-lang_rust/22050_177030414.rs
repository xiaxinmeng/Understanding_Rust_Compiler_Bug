 rust
let mut buf=[0u8;32];
(&mut buf[..]).write_all(&b"test");
