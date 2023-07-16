
error[E0119]: conflicting implementations of trait `std::ops::Deref` for type `&_`:
 --> test.rs:5:1
  |
5 | impl<Foo> Deref for Foo { }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: conflicting implementation in crate `core`

error[E0210]: type parameter `Foo` must be used as the type parameter for some local type (e.g. `MyStruct<T>`); only traits defined in the current crate can be implemented for a type parameter
 --> test.rs:5:1
  |
5 | impl<Foo> Deref for Foo { }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error(s)
