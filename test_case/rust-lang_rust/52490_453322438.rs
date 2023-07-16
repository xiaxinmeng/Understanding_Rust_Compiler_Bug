
error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
 --> src/main.rs:5:33
  |
4 |     let v = vec![0];
  |         - help: consider changing this to be mutable: `mut v`
5 |     foo(move || { let _x = &mut v[..]; });
  |                                 ^ cannot borrow as mutable
