rust
let mut slice = [1, 2, 2, 3, 3, 2];

let dedup = slice.partition_dedup();
assert_eq!(dedup, [1, 2, 3, 2]);

let duplicates = slice[dedup.len()..];
assert_eq!(duplicates, [3, 2]);
