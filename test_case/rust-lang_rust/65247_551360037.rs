
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
  --> src/main.rs:11:6
   |
11 | impl<T> From<BadOption<T>> for Option<T> {
   |      ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: only traits defined in the current crate can be implemented for a type parameter
