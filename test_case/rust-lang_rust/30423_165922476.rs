 rust
slice.binary_search_by(|k| k.key.cmp(&val));

// vs

slice.binary_search_by_key(&val, |k| &k.key);
