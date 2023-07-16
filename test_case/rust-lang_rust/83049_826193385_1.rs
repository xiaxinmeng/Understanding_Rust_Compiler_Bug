
error[E0603]: function `baz` is private
 --> src/lib.rs:3:21
  |
3 |         super::bar::baz()
  |                     ^^^ private function
  |
note: the function `baz` is defined here
 --> src/lib.rs:8:5
  |
8 |     fn baz() {}
  |     ^^^^^^^^
