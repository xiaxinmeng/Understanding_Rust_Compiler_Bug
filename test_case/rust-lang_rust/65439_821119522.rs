rust
let f = File::OpenOptions::new().create(false).write(true).open("abc")?;
