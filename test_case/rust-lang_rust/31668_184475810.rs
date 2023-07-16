 rust
let file = File::from_raw_fd(libc::open(...)); // non LFS open
file.set_len(u32::MAX + 1);
