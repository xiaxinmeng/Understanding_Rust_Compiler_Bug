
error: unreachable statement
  --> src/main.rs:13:9
   |
8  | /         match e1 {
9  | |             &1 => {
10 | |                 continue 'label;
11 | |             }
12 | |         }
   | |_________- any code following this `match` expression is unreachable, as all arms diverge
13 |           println!("e1: {:?}", e1);
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
   |
note: the lint level is defined here
  --> src/main.rs:1:9
   |
1  | #![deny(unreachable_code)]
   |         ^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0004]: non-exhaustive patterns: `&i32::MIN..=0_i32` and `&2_i32..=i32::MAX` not covered
 --> src/main.rs:8:15
  |
8 |         match e1 {
  |               ^^ patterns `&i32::MIN..=0_i32` and `&2_i32..=i32::MAX` not covered
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
  = note: the matched value is of type `&i32`
