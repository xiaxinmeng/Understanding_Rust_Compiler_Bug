rust
//this is ok to do with vec:
let vec = vec![1, 1, 2, 3];
vec.dedup()
// can use vec, no problem
// same case with slices
let mut slice = [1, 1, 2, 3];
slice.dedup_in_place();
// uses slice, oops
// or worst:
let mut slice = [1, 1, 2, 3];
let new = slice.dedup_in_place();
//  one may use slice, ignore new and refer to “removed” elements
