 rust
let mut parts = slice.windows_mut(4);
let part1 = parts.next().unwrap();
let part2 = parts.next().unwrap();
// part1/part2 overlap
