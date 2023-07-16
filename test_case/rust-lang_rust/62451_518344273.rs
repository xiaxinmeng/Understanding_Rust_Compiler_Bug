rust
let a = Arc::new(0);
let r1 = Arc::get_mut_unchecked();
let r2 = Arc::get_mut_unchecked();
// Two aliasing raw pointers, no problem.
*r1 = 5;
assert_eq!(*r2, 5);
