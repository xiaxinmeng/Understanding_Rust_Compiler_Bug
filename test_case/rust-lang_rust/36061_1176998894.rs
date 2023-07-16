
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
 --> src/lib.rs:2:6
  |
2 | impl<T: Trait> Drop for T {
  |      ^ type parameter `T` must be used as the type parameter for some local type
  |
  = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
  = note: only traits defined in the current crate can be implemented for a type parameter

error[E0120]: the `Drop` trait may only be implemented for structs, enums, and unions
 --> src/lib.rs:2:25
  |
2 | impl<T: Trait> Drop for T {
  |                         ^ must be a struct, enum, or union
