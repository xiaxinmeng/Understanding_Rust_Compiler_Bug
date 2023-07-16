rust
let src = [1, 2, 3, 4];
let mut dst = [0, 0];

dst.copy_from_slice(&src[2..]);

assert_eq!(src, [1, 2, 3, 4]);
assert_eq!(dst, [3, 4]);
