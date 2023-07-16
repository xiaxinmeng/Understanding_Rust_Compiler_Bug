
error[E0277]: the size for values of type `Self` cannot be known at compilation time
   --> src/lib.rs:2:46
    |
2   |     const SIZE: usize = core::mem::size_of::<Self>();
    |                                              ^^^^ doesn't have a size known at compile-time
    |
help: consider further restricting `Self`
    |
1   | trait Foo: Sized<T> {
    |          ^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground`
