
warning: unnecessary parentheses around assigned value
  --> ./src/main.rs:35:18
   |
35 |         let _b = (foo_b());
   |                  ^^^^^^^^^ help: remove these parentheses
   |
   = note: `#[warn(unused_parens)]` on by default                // This is the children part.
