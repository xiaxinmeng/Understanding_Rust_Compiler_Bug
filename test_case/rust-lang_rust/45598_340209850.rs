
   Compiling playground v0.0.1 (file:///playground)
error[E0277]: the trait bound `T: std::marker::Sized` is not satisfied
 --> src/main.rs:3:18
  |
3 | struct U<T>([u8; mem::size_of::<T>()], PhantomData<T>);
  |                  ^^^^^^^^^^^^^^^^^ `T` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `T`
  = help: consider adding a `where T: std::marker::Sized` bound
  = note: required by `std::mem::size_of`

error: aborting due to previous error

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
