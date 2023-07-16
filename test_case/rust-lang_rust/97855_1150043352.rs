rust
let inp = stdin().lock();
let inp = ManuallyDrop::new(unsafe { File::from_raw_fd(inp.as_raw_fd()) });
inp.read_exact(&mut buf)?;
drop(inp);

// spawn command here.
