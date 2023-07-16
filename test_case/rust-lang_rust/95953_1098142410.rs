rust
let mut x: i32 = 0;
let r = &x;
let a = [r; 0];
x = 5;
let _b = a;
