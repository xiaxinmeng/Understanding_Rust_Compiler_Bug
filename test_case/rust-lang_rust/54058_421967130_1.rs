rust
let slice = &mut [1, 2, 2, 3, 3, 2];
let slice2 = slice;

slice.dedup();
assert_eq!(slice, &mut [1, 2, 3, 2]);

let duplicates = &slice2[dedup.len()..];
assert_eq!(duplicates, &[3, 2]);
