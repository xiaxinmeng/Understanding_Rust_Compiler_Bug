rust
if let Err(InvalidDigit(i)) = "1a0".parse::<i32>().map(ParseIntError::kind) {
     println!("Parsing failed at index: {}", i);
}
