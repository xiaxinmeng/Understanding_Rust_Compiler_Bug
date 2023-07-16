rust
// example 2
let x = &mut 0;
let shared = &*x;
let y: *const i32 = x;
let _val = *shared;
unsafe { *(y as *mut i32) = 1; }
