rust
["1", "foo", "3"]
    .iter()
    .filter_map(|n| n.parse::<i32>())
    .collect()
