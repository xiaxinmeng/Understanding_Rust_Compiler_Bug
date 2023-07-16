 rust
let a = box 1;
let b = || { println!("{}", a) };
drop(b);
// no memory should leak
