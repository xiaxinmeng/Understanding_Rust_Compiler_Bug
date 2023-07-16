rust
let file = OpenOptions::new().read(true).custom_flags(libc::O_PATH).open(".")?;
let new_file = file.try_clone()?; // gives EBADF
