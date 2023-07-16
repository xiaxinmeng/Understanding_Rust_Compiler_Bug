rust
while let Some(_) = reader.fill_buf2()? {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    println!("{:?}", line);
}
