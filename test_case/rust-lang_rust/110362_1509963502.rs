rust
let mut left = vec![0, 1, 2];
let right = vec![1, 2, 3];
let duplicate_part = &mut left[n..];
left.extend_from_slice(right.strip_prefix(duplicate_part).unwrap());
