rust
#![feature(array_from_fn)]
pub fn str_to_char_array<const N: usize>(s: &str) -> Option<[char; N]> {
    let mut c = s.chars();
    let a = std::array::try_from_fn(|_| c.next())?;
    if c.next().is_none() {
        Some(a)
    } else {
        None
    }
}
