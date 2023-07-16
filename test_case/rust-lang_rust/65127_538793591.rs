rust
pub fn foo(c: char) -> bool {
    match c as u8 {
        65..=90 => true,
        _ => false
    }
}

pub fn bar(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        _ => false,
    }
}

pub fn baz(c: char) -> bool {
    (c as u8).is_ascii_uppercase()
}
