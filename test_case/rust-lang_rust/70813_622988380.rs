
error[E0637]: `&` without an explicit lifetime name cannot be used here
 --> file55.rs:2:36
  |
2 | pub trait FooExt where for<'b> &'b &mut Self: Foo {}
  |                                    ^ explicit lifetime name needed here

error[E0277]: the trait bound `for<'b> &'b &'static mut T: Foo` is not satisfied
 --> file55.rs:3:9
  |
2 | pub trait FooExt where for<'b> &'b &mut Self: Foo {}
  |                                               --- required by this bound in `FooExt`
3 | impl<T> FooExt for T where for<'a> &'a mut T: Foo {}
  |         ^^^^^^ the trait `for<'b> Foo` is not implemented for `&'b &'static mut T`

error: aborting due to 2 previous errors
