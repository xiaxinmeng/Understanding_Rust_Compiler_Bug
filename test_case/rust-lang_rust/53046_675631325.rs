
error[E0642]: patterns aren't allowed in functions without bodies
 --> src/lib.rs:9:12
  |
9 |     fn bar((x, y): (i32, i32));
  |            ^^^^^^ pattern not allowed in function without body
