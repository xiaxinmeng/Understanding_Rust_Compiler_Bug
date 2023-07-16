 rust
let mut x = (1i32, 2i64);
let y =     (2i32, 3i64);
let fail =  (2i64, 5i32);

x = y;    // Works because signature is the same
x = fail; // Correct types, wrong order. This doesn't work.
