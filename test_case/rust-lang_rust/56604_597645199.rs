rust
let x = &mut 0;
let shared = &*x;
let y = x as *const i32; // if we use *mut here instead, this stops compiling
let _val = *shared;
