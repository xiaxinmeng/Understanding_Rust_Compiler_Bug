
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
 --> src/lib.rs:4:13
  |
4 | unsafe impl<T: Private> Send for T {
  |             ^ type parameter `T` must be used as the type parameter for some local type
  |
  = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
  = note: only traits defined in the current crate can be implemented for a type parameter
