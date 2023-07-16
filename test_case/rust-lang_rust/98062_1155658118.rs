
error[E0405]: cannot find trait `TryFrom` in this scope
 --> eg1.rs:9:6
  |
9 | impl TryFrom<()> for S {
  |      ^^^^^^^ not found in this scope
  |
  = note: 'std::convert::TryFrom' is included in the prelude starting in Edition 2021
help: consider importing this trait
  |
1 | use std::convert::TryFrom;
  |

error: aborting due to previous error

