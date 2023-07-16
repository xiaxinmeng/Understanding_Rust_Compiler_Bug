rust
["1", "foo", "3"]
    .iter()
    .filter(|n| n.parse::<i32>().is_some())
    .map(|n| n.parse::<i32>().unwrap())
    .collect()
