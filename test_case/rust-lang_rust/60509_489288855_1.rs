text
error[E0596]: cannot borrow `__arg0.0` as mutable, as `__arg0` is not declared as mutable
  --> code\src\lib.rs:17:15
   |
17 | async fn foo((ref mut c, ref d): (A, A)) {}
   |              -^^^^^^^^^--------
   |              ||
   |              |cannot borrow as mutable
   |              help: consider changing this to be mutable: `mut __arg0`
