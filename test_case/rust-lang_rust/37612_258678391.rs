 rust
if let Ok(ref data) = self.cache.read() {
    if data.len() > to {
        Some(data[to].clone())
    } else {
        None
    }
} else {
    None
}
