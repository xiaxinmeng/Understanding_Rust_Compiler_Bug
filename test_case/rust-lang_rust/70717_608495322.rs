rust
let a = 0;
let ref mut b = {a};
*b += 1;
assert_eq!(a, 0);
