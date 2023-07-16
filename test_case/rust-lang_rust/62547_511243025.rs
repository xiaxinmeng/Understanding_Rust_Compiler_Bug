rust
let mut x = 0;    // (1)
let p = &mut x;   // (2)
*p = 1;           // (3)
x = 2;            // (4)
x
