rust
["1", "foo", "3"]
    .iter()
    .map(|n| n.parse::<i32>())
    .filter(|n| n.is_some())
    .collect()  // [Some(1), Some(3)]
