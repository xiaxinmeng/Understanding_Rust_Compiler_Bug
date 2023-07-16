rust
    fs::write("foo.txt", b"Hello, world!")?;

    let file = File::open("foo.txt")?;
    let output = Command::new("rev")
        .stdin(Stdio::from(file))  // convert the File into a Stdio
        .output()?;

    assert_eq!(output.stdout, b"!dlrow ,olleH");
