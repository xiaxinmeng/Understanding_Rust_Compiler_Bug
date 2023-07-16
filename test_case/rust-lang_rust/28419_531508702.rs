
error[E0596]: cannot borrow data in a dereference of `B` as mutable
  --> src/main.rs:17:5
   |
17 |     b.yow();
   |     ^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `B`
