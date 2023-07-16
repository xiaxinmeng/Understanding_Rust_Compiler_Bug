rust
let x = 0;
let y = &x;
let p = unsafe { core::mem::transmute::<&&i32,&&mut i32>(&y) };
let _r = &**p;
