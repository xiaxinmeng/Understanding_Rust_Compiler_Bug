rust
pub fn minus_match(x: bool, y: bool) -> Ordering {
    match x as i8 - y as i8 {
        -1 => Ordering::Less,
        0 => Ordering::Equal,
        1 => Ordering::Greater,
        _ => unsafe { std::hint::unreachable_unchecked() }
    }
}
