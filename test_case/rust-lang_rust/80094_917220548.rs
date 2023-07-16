rust
// Note that an array would implement the fixed iterator trait (output equals the array itself).
let arr = [1, 2, 3];
// You could use the map method to change the array values.
let mapped = arr.map(|v| v*2);
// Then you could either turn the mapped fixed iterator into an array
let double_arr: [i32; 3] = mapped.array();
// or you could turn it into an iterator (note that the iterator values are of type `i32`, not `&i32`).
let sum: i32 = mapped.iter().sum();
// Note that it was used a second time here because the mapped array implements `Copy`.
