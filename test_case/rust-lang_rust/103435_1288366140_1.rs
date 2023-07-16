console
warning: unnecessary parentheses around pattern
 --> src/main.rs:2:8
  |
2 |     for(_x) in 1..10 { }
  |        ^  ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     for(_x) in 1..10 { }
2 +     for_x in 1..10 { }
  |
