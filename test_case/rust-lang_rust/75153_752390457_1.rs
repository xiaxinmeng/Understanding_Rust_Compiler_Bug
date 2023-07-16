
error[E0277]: the size for values of type `T` cannot be known at compilation time
   --> src/main.rs:4:28
    |
3   | pub const fn is_zst<T: ?Sized>() -> usize {
    |                     - this type parameter needs to be `Sized`
4   |     if std::mem::size_of::<T>() == 0 {
    |                            ^ doesn't have a size known at compile-time

error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> src/main.rs:12:12
   |
11 | pub struct AtLeastByte<T: ?Sized> {
   |                        - this type parameter needs to be `Sized`
12 |     value: T,
   |            ^ doesn't have a size known at compile-time
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
12 |     value: &T,
   |            ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
12 |     value: Box<T>,
   |            ^^^^ ^

error: aborting due to 2 previous errors; 1 warning emitted
