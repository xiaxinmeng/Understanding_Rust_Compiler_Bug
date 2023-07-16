
error: generic parameters may not be used in const operations
  --> /home/runner/work/glacier/glacier/ices/75153.rs:11:24
   |
11 |     pad: [u8; is_zst::<T>()],
   |                        ^ cannot perform const operation using `T`
   |
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions

error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /home/runner/work/glacier/glacier/ices/75153.rs:10:12
   |
9  | pub struct AtLeastByte<T: ?Sized> {
   |                        - this type parameter needs to be `Sized`
10 |     value: T,
   |            ^ doesn't have a size known at compile-time
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
10 |     value: &T,
   |            ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
10 |     value: Box<T>,
   |            ^^^^ ^

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
