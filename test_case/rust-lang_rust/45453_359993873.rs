
error[E0283]: type annotations required: cannot resolve `_: std::convert::Into<std::string::String>`
 --> src/main.rs:3:5
  |
3 |     func1(42);
  |     ^^^^^
  |
note: required by `func1`
 --> src/main.rs:1:1
  |
1 | fn func1<T: Into<String>>(x: i32) {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
