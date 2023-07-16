rust
let x = 0i8;
assert_eq!(x.signum() as f64, (x as f64).signum()); // ERROR
