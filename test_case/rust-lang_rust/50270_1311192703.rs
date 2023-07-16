
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
 --> src/lib.rs:3:10
  |
3 | impl<'a, T: Trait> From<&'a T> for () {
  |          ^ type parameter `T` must be used as the type parameter for some local type
  |
  = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
  = note: only traits defined in the current crate can be implemented for a type parameter
