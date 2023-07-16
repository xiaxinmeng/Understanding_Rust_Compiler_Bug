 rust
let s = Wrapped(1i32) >> (|x| x + 1) >> (|x| 2*x) >> (|x: i32| x.to_string());
