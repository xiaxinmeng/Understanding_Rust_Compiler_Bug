
error: couldn't read src/data/q24.data: No such file or directory (os error 2)
  --> src/main.rs:14:22
   |
14 | static INPUT: &str = include_str!("data/q24.data");
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in the macro `include_str` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find macro `q_impl` in this scope
   --> src/main.rs:300:1
    |
300 | q_impl!("24");
    | ^^^^^^
