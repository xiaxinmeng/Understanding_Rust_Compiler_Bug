 rust
#![crate_type="lib"]

pub fn split_to(input: &[u8]) -> &[u8] {
    match input.iter().position(|x| *x == b' ') {
        Some(i) => &input[..i],
        None => &[],
    }
}

pub fn split_from(input: &[u8]) -> &[u8] {
    match input.iter().position(|x| *x == b' ') {
        Some(i) => &input[i..],
        None => &[],
    }
}

pub fn find(input: &[u8]) -> Option<&u8> {
    match input.iter().position(|x| *x == b' ') {
        Some(i) => Some(&input[i]),
        None => None,
    }
}
