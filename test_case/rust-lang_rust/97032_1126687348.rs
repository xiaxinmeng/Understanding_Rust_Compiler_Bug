plain
ii..i..i......i......i.ii..i......i......i.ii....................i........i..i.......... 88/108
F...i...............
failures:

---- src/builtin.rs - builtin::UNUSED_MACRO_RULES (line 791) stdout ----
error: 2nd rule of macro `unused_empty` is never used
  |
  |
5 |     () => { println!("empty") }; // This rule is used
  |
note: the lint level is defined here
 --> src/builtin.rs:792:8
  |
