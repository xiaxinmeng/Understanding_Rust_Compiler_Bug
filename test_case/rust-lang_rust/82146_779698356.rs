rust
if cache.contains_key(&input) {
    match cache.get(&input) {
        Some(cached) => return Cow::Borrowed(cached),
        None => unreachable!(),
    }
}
