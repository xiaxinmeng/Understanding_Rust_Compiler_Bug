
error: this file contains an unclosed delimiter
 --> /home/dwrensha/Desktop/bug.rs:1:17
  |
1 | fn f(){(print!(á
  |       --      - ^
  |       ||      |
  |       ||      unclosed delimiter
  |       |unclosed delimiter
  |       unclosed delimiter

error: format argument must be a string literal
 --> /home/dwrensha/Desktop/bug.rs:1:16
  |
1 | fn f(){(print!(á
  |                ^
  |
help: you might be missing a string literal to format with
  |
1 | fn f(){(print!("{}", á
  |                +++++

warning: unnecessary parentheses around block return value
 --> /home/dwrensha/Desktop/bug.rs:1:8
  |
1 | fn f(){(print!(á
  |        ^        ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
1 - fn f(){(print!(á
1 + fn f(){print!(á
  | 

error: aborting due to 2 previous errors; 1 warning emitted

