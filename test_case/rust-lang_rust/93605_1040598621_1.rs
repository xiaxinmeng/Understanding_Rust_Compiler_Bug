text
warning: unclosed HTML tag `i32`
 --> test.rs:6:31
  |
6 | /// This [test][ExistentStruct<i32>] thing!
  |                               ^^^^^
  |
note: the lint level is defined here
 --> test.rs:1:9
  |
1 | #![warn(rustdoc::invalid_html_tags)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try marking as source code
  |
6 | /// This [test][`ExistentStruct<i32>`] thing!
  |                 +                   +

warning: 1 warning emitted
