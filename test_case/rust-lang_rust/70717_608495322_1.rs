rust
// `mut` qualifier is necessary, also not observed by the lint.
let mut a = 0;
let ref mut b = a;
*b += 1;
assert_eq!(a, 1);
