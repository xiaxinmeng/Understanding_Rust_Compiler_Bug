
warning: unnecessary parentheses around block return value
 --> src/lib.rs:2:5
  |
2 |     ({ u8::from(a) } + { u8::from(b) })
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
  |
  = note: `#[warn(unused_parens)]` on by default
