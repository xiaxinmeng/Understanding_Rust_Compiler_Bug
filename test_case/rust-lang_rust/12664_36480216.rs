 rust
let f: u8 = '\n' as u8;
let b: u8 = match f as char {
    '\n' => 1,
    _ => 2
};
