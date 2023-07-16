
error[E0432]: unresolved imports: `std::time::foo`, `std::time::bar`, `std::time::buzz`, `std::time::abc`, `std::time::def`
 --> /tmp/test.rs:1:17
  |
1 | use std::time::{foo, bar, buzz};
  |                 ^^^  ^^^  ^^^^
2 | use std::time::{abc, def};
  |                 ^^^  ^^^

warning: unused imports: `bar`, `buzz`, `foo`
 --> /tmp/test.rs:1:17
  |
1 | use std::time::{foo, bar, buzz};
  |                 ^^^  ^^^  ^^^^
  |
  = note: #[warn(unused_imports)] on by default

warning: unused imports: `abc`, `def`
 --> /tmp/test.rs:2:17
  |
2 | use std::time::{abc, def};
  |                 ^^^  ^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
