rust
let s = system_time.to_string();
let year: u32 = s[..4].parse().unwrap();
let month: u32 = s[5..7].parse().unwrap();
let day: u32 = s[8..10].parse().unwrap();
// etc.