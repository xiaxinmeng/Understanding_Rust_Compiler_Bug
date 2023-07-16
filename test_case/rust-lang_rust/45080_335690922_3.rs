
warning: unnecessary parentheses around assigned value
  --> c:\path\to\file\suggestions.rs:17:21
   |
17 |         let mut a = (1); // should suggest no `mut`, no parens
   |                     ^^^ help: remove these parentheses
   |
   = note: #[warn(unused_parens)] on by default
