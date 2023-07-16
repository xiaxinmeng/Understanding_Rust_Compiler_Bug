rs
error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
 --> poc.rs:6:5
  |
4 |         let [ref y, ref mut z @ ..] = x;
  |                                       - calling `f` requires mutable binding due to mutable borrow of `x`
5 |     };
6 |     f();
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
3 |     let mut f = || {
  |     
