
error[E0119]: conflicting implementations of trait `std::clone::Clone` for type `&_`:
 --> test.rs:1:1
  |
1 | impl<T> Clone for T { fn clone(&self) -> T { panic!() } }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: conflicting implementation in crate `core`

error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`); only traits defined in the current crate can be implemented for a type parameter
 --> test.rs:1:1
  |
1 | impl<T> Clone for T { fn clone(&self) -> T { panic!() } }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error(s)
