text
error[E0277]: expected a `std::ops::FnMut<(_,)>` closure, found `Foo`
  --> src/main.rs:28:5
   |
28 |     repro(Foo);
   |     ^^^^^ expected an `FnMut<(_,)>` closure, found `Foo`
   |
   = help: the trait `std::ops::FnMut<(_,)>` is not implemented for `Foo`
   = note: required because of the requirements on the impl of `Invoke<_>` for `Foo`
note: required by `repro`
  --> src/main.rs:23:1
   |
23 | fn repro<A, I: Invoke<A>>(_: I) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
