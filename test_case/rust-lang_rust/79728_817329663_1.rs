rust
if let Err(error) = "1a0".parse::<i32>() {
    if let Err(InvalidDigit) = error.kind() {
        let i = error.index().unwrap(); // Forced to unwrap
        println!("Parsing failed at index: {}", i);
    }
}
