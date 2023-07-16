
error: this file contains an un-closed delimiter
 --> 44021.rs:3:19
  |
3 |    fn f() {|x, y}
  |                   ^
  |
help: did you mean to close this delimiter?
 --> 44021.rs:2:15
  |
2 | impl MyStruct {
  |               ^

error: expected one of `:`, `@`, or `|`, found `}`
 --> 44021.rs:3:17
  |
3 |    fn f() {|x, y}
  |                 ^ expected one of `:`, `@`, or `|` here
