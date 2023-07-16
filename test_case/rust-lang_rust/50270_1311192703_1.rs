
error[E0210]: type parameter `T` must be used as the type parameter for a local type
 --> src/lib.rs:3:10
  |
3 | impl<'a, T: Trait> From<&'a T> for () {
  |          ^         ---- this is not a local type
  |          |
  |          type parameter `T` must be used as the type parameter for some local type
  |
note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
 --> src/lib.rs:3:10
  |
3 | impl<'a, T: Trait> From<&'a T> for () {
  |                    ^^^^^^^^^^^     ^^ this type is not local
  |                    |
  |                    this trait is not local and it doesn't reference a local type
  |          
note: only traits defined in the current crate can be implemented for a type parameter
 --> src/lib.rs:3:10
  |
3 | impl<'a, T: Trait> From<&'a T> for () {
  |                         ^^^^^ this should be a local type, not a type parameter
