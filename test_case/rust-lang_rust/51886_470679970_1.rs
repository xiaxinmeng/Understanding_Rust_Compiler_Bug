
warning: variable does not need to be mutable
 --> src/lib.rs:3:53
  |
3 | fn foo<F: FnMut(), W: Deref<Target = F> + DerefMut>(mut f: W) {
  |                                                     ----^
  |                                                     |
  |                                                     help: remove this `mut`
  |
  = note: #[warn(unused_mut)] on by default

error[E0596]: cannot borrow data in a `&` reference as mutable
 --> src/lib.rs:4:5
  |
4 |     f()
  |     ^ cannot borrow as mutable
