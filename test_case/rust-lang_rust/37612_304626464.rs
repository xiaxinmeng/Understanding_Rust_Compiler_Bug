Rust

{
    match self.cache.read() { // <-- direct pattern matching
        Ok(ref data) => Some(data)
        _ => None,
    }
}.map(|data| {
    // use `data` - the lock better be held
})
