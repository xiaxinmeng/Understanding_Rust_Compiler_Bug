 rust
#[inline]
pub fn partial_min<T: PartialOrd>(v1: T, v2: T) -> Option<T> {
    match v1.partial_cmp(&v2) {
        None => None,
        Some(Less) => Some(v1),
        _ => Some(v2),
    }
}

#[inline]
pub fn partial_max<T: PartialOrd>(v1: T, v2: T) -> Option<T> {
    match v1.partial_cmp(&v2) {
        None => None,
        Some(Greater) => Some(v1),
        _ => Some(v2)
    }
}
