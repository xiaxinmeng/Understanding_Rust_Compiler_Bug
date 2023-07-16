rust
error[E0309]: the parameter type `T` may not live long enough
  --> src/main.rs:22:5
   |
22 |     fn send<'a: 'a>(_message: <T as By<'a, Ref>>::Type) {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound...
   |
21 | impl<T: 'a> Transmit<T, Ref> for () {
   |       ++++
