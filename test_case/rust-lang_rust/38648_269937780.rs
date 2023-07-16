
let mut f = File::create("foo.txt")?;
f.write_all(b"Hello, world!")?;
