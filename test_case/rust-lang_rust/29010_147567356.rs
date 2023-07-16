 rust
let i: isize = -5;
let j: usize =  5;
let x = [0; 2];
let a = x[i]; // panic (out-of-bounds low)
let a = x[j]; // panic (out-of-bounds high)
