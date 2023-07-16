
error[E0210]: type parameter `<Q as TR>::A` must be used as the type parameter for some local type (e.g., `MyStruct<<Q as TR>::A>`)
 --> src/lib.rs:9:1
  |
9 | impl From<&str> for QA {
  | ^^^^^^^^^^^^^^^^^^^^^^ type parameter `<Q as TR>::A` must be used as the type parameter for some local type
  |
  = note: only traits defined in the current crate can be implemented for a type parameter

error: aborting due to previous error
