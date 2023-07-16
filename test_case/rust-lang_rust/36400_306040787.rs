
error[E0596]: cannot borrow immutable `Box` content `*x` as mutable
 --> test.rs:5:12
  |
4 |     let x = Box::new(3);
  |         - consider changing this to `mut x`
5 |     f(&mut *x);
  |            ^^ cannot borrow as mutable

error: aborting due to previous error(s)
