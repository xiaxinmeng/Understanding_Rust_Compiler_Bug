
error[E0596]: cannot borrow data in a dereference of `impl std::ops::DerefMut` as mutable
  --> src/lib.rs:14:25
   |
14 |         let rgba_icon = (*get_icon)();
   |                         ^^^^^^^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `impl std::ops::DerefMut`
