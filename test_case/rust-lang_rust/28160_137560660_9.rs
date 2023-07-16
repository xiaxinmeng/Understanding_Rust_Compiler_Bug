 Rust
x.get_mut().push(x[0].len());
// equiv
let t0 = x.get_mut();
let len = x[0].len(); //~ ERROR cannot borrow
t0.push(len);
