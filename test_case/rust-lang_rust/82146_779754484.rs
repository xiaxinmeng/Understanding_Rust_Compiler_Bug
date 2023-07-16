rust
if cache.contains_key(&input) {
    return Cow::Borrowed(&cache[&input]);
}
