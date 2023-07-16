
error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
 --> src/lib.rs:3:9
  |
1 | fn test<F: FnMut()>(f: F) {
  |                     - help: consider changing this to be mutable: `mut f`
2 |     let _f = move || {
3 |         f()
  |         ^ cannot borrow as mutable
