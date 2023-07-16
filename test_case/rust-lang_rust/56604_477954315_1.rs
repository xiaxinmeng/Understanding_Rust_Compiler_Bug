rust
// example 1
let x = &mut 0;
let y: *const i32 = x;
unsafe { *(y as *mut i32) = 1; }
