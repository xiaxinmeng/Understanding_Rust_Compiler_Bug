rust
let mut line = String::new();
while let Ok(n) = reader.read_line(&mut line)? {
    if n == 0 {
        break;
    }
    println!("{:?}", line);
    line.clear();
}
