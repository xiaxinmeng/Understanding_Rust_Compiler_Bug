Rust
let mut a = 0;
let mut b = 0;
let mut x: &mut u32 = &mut a;
let y = &mut x;
*y = &mut b; // this borrow should be active
// look at me, no use of `*y`, but:
use(x, &b); // should be illegal

