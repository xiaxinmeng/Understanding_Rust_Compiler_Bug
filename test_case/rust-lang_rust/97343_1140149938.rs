
error[E0109]: type arguments are not allowed for this type
 --> f41.rs:4:19
  |
3 | #[derive(Debug)]
  |          ----- in this derive macro expansion
4 | struct Irrelevant<Irrelevant> {
  |                   ^^^^^^^^^^ type argument not allowed
  |
note: type parameter `Irrelevant` defined here
 --> f41.rs:4:19
  |
4 | struct Irrelevant<Irrelevant> {
  |                   ^^^^^^^^^^
  = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
