
 --> src/main.rs:7:18
  |
4 | fn is_average<T: num_integer::Average>() {}
  |                  -------------------- required by this bound in `is_average`
...
7 |     is_average::<Vec2<T>>();
  |                  ^^^^^^^ the trait `Integer` is not implemented for `vek::Vec2<T>`
  |
  = note: required because of the requirements on the impl of `Average` for `vek::Vec2<T>`

error[E0277]: no implementation for `T >> T`
 --> src/main.rs:7:5
  |
4 | fn is_average<T: num_integer::Average>() {}
  |                  -------------------- required by this bound in `is_average`
...
7 |     is_average::<Vec2<T>>();
  |     ^^^^^^^^^^^^^^^^^^^^^ no implementation for `T >> T`
  |
  = note: required because of the requirements on the impl of `Shr<usize>` for `vek::Vec2<T>`
  = note: required because of the requirements on the impl of `Average` for `vek::Vec2<T>`
help: consider restricting type parameter `T`
  |
6 | fn foo<T: Shr>() {
  |         ^^^^^


thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
