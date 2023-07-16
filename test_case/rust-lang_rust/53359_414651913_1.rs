
error[E0432]: unresolved import `std::time::foo`
 --> /tmp/test.rs:1:17
  |
1 | use std::time::{foo, bar, buzz};
  |                 ^^^ no `foo` in `time`

error[E0432]: unresolved import `std::time::bar`
 --> /tmp/test.rs:1:22
  |
1 | use std::time::{foo, bar, buzz};
  |                      ^^^ no `bar` in `time`

error[E0432]: unresolved import `std::time::buzz`
 --> /tmp/test.rs:1:27
  |
1 | use std::time::{foo, bar, buzz};
  |                           ^^^^ no `buzz` in `time`

error[E0432]: unresolved import `std::time::abc`
 --> /tmp/test.rs:2:17
  |
2 | use std::time::{abc, def};
  |                 ^^^ no `abc` in `time`

error[E0432]: unresolved import `std::time::def`
 --> /tmp/test.rs:2:22
  |
2 | use std::time::{abc, def};
  |                      ^^^ no `def` in `time`

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

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0432`.
