
error[E0405]: cannot find trait `A` in this scope
  --> src/lib.rs:64:27
   |
64 |   my_field: Vec<my_module:A>,
   |                           ^ not found in this scope
   |
help: you might have meant to write a path instead of an associated type bound
   |
64 |   my_field: Vec<my_module::A>,
   |                          ~~
help: you might be missing a type parameter
   |
63 | struct MyStruct2<A> {
   |                 +++

error[E0658]: associated type bounds are unstable
  --> src/lib.rs:64:17
   |
64 |   my_field: Vec<my_module:A>,
   |                 ^^^^^^^^^^^
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable

error[E0107]: this struct takes at least 1 generic argument but 0 generic arguments were supplied
  --> src/lib.rs:64:13
   |
64 |   my_field: Vec<my_module:A>,
   |             ^^^ expected at least 1 generic argument
   |
help: add missing generic argument
   |
64 |   my_field: Vec<T, my_module:A>,
   |                 ++

error[E0229]: associated type bindings are not allowed here
  --> src/lib.rs:64:17
   |
64 |   my_field: Vec<my_module:A>,
   |                 ^^^^^^^^^^^ associated type not allowed here
