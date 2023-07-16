
error[E0720]: cannot resolve opaque type
 --> src/lib.rs:1:17
  |
1 | fn bar() -> Vec<impl Copy> {
  |                 ^^^^^^^^^ cannot resolve opaque type
2 |     panic!()
  |     -------- this returned value is of `!` type
  |
  = help: this error will resolve once the item's body returns a concrete type

error[E0277]: `()` is not a future
 --> src/lib.rs:1:13
  |
1 | fn foo() -> impl std::future::Future<Output = i32> {
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` is not a future
2 |     unimplemented!();
  |     ----------------- consider removing this semicolon
  |
  = help: the trait `Future` is not implemented for `()`

error[E0720]: cannot resolve opaque type
 --> src/lib.rs:5:34
  |
5 |   fn make_marker() -> MarkerHolder<impl std::fmt::Display> {
  |                                    ^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
6 | /     MarkerHolder {
7 | |         _marker: std::marker::PhantomData,
8 | |     }
  | |_____- returning here with type `MarkerHolder<impl std::fmt::Display>`

