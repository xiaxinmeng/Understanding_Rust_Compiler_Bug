rust
loop {
    match (self.next(), other.next()) {
        (None, None) => return true,
        (None, _) | (_, None) => return false,
        (Some(x), Some(y)) => if x != y { return false },
    }
}
