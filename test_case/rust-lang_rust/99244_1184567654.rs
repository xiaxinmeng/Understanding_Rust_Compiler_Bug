plain
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::scan (line 1403) stdout ----
error: unnecessary parentheses around `if` condition
  --> src/iter/traits/iterator.rs:1411:8
   |
11 |     if (*state > 6) {
   |        ^          ^
note: the lint level is defined here
  --> src/iter/traits/iterator.rs:1401:9
   |
1  | #![deny(warnings)]
1  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unused_parens)]` implied by `#[deny(warnings)]`
help: remove these parentheses
   |
11 -     if (*state > 6) {
11 +     if *state > 6 {

error: aborting due to previous error

Couldn't compile the test.
