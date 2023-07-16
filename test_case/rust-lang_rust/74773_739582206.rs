rust
let (key, value) = option.split_once('=').unwrap();
let value = if value.is_empty() { None } else { Some(value) };
