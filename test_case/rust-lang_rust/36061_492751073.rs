
error[E0119]: conflicting implementations of trait `std::ops::Drop` for type `std::boxed::Box<_>`:
 --> src/lib.rs:2:1
  |
2 | impl<T: Trait> Drop for T {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: conflicting implementation in crate `alloc`:
          - impl<T> std::ops::Drop for std::boxed::Box<T>
            where T: ?Sized;
  = note: downstream crates may implement trait `Trait` for type `std::boxed::Box<_>`

error[E0120]: the Drop trait may only be implemented on structures
 --> src/lib.rs:2:25
  |
2 | impl<T: Trait> Drop for T {
  |                         ^ implementing Drop requires a struct

error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
 --> src/lib.rs:2:1
  |
2 | impl<T: Trait> Drop for T {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `T` must be used as the type parameter for some local type
  |
  = note: only traits defined in the current crate can be implemented for a type parameter

error: aborting due to 3 previous errors
