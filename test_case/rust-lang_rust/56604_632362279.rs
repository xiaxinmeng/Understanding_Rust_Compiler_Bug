rust
let x = 22;
let p: *const i32 = raw_ref!(x);
let q: *mut i32 = p as *mut i32;
*q += 1;
