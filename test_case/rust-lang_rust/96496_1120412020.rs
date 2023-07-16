rust
let a: u128 = 3; // a is a typed `u128`, Ok!
let b = 4; // b is `i32` but needs to be inferred
let c = a; // c is `u128` but needs to be inferred

assert!(a == 10 && b = 10 && c == 10);
