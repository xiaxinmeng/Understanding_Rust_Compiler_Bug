console
error: this file contains an unclosed delimiter
  --> src/test/ui/parser/deli-ident-issue-1.rs:27:65
   |
7  | impl dyn Demo {
   |               - unclosed delimiter
...
19 |         && let Some(c) = num {
   |                              - this delimiter might not be properly closed...
...
24 |     }
   |     - ...as it matches this but it has different indentation
...
27 | fn main() { } //~ ERROR this file contains an unclosed delimiter
   |                                                                 ^
