rust
let file = File::open("foo.txt")?;
let output = Command::new("rev")
    .stdin(file)
    .output()?;
