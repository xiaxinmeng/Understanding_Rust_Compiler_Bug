rust
pub fn last(xs: &[u32]) -> Option<u32> {
    match xs.len() {
        0 => None,
        1 => Some(xs[0]),
        _ => last(&xs[1..])
    }
}
