 rust
let mut chars = reader.chars();
loop {
    for c in chars {
        // ...
    }
    if chars.was_valid_utf8() { break; }
    println!("Encountered invalid byte sequence.");
}
