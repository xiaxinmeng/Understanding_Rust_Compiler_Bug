rust
error[E0277]: the size for values of type `dyn My` cannot be known at compilation time
   --> <source>:19:20
    |
19  |     println!("{}", std::mem::align_of::<Wrapper<dyn My>>());
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: within `Wrapper<dyn My>`, the trait `Sized` is not implemented for `dyn My`
note: required because it appears within the type `Wrapper<dyn My>`
   --> <source>:5:12
    |
5   | pub struct Wrapper<T: ?Sized>(i8, T);
    |            ^^^^^^^
note: required by a bound in `align_of`
