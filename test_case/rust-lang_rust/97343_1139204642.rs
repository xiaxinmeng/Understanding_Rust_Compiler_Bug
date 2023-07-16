
error[E0574]: expected struct, variant or union type, found type parameter `Irrelevant`
 --> <source>:1:10
  |
1 | #[derive(Debug)]
  |          ^^^^^ not a struct, variant or union type
  |
  = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0109]: type arguments are not allowed for this type
 --> <source>:1:10
  |
1 | #[derive(Debug)]
  |          ^^^^^ type argument not allowed
  |
  = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0210]: type parameter `Irrelevant` must be used as the type parameter for some local type (e.g., `MyStruct<Irrelevant>`)
 --> <source>:1:10
  |
1 | #[derive(Debug)]
  |          ^^^^^ type parameter `Irrelevant` must be used as the type parameter for some local type
  |
  = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
  = note: only traits defined in the current crate can be implemented for a type parameter

error: aborting due to 3 previous errors
