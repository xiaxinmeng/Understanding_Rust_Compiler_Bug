
error[E0637]: `'_` cannot be used here
 --> src/lib.rs:3:20
  |
3 | pub trait Captures<'_> {
  |                    ^^ `'_` is a reserved lifetime name

error: lifetime parameter `'_` never used
 --> src/lib.rs:3:20
  |
3 | pub trait Captures<'_> {
  |                   -^^- help: elide the unused lifetime
