rust
let x = vec![1,2,3];
let xptr = &x as *const _;
move_elsewhere(x);
unsafe { /* look at *xptr. */ }
