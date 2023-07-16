
error[E0782]: trait objects must include the `dyn` keyword
 --> src/main.rs:1:23
  |
1 | fn cause() -> Option<&std::fmt::Display> {}
  |                       ^^^^^^^^^^^^^^^^^
  |
help: add `dyn` keyword before this trait
  |
1 | fn cause() -> Option<&dyn std::fmt::Display> {}
  |                       +++
