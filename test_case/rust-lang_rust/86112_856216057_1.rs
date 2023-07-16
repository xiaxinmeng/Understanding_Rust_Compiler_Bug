
error: unreachable pattern
  --> src/main.rs:6:13
   |
6  |             _ => false
   |             ^ unreachable pattern
...
17 |     if matches!(1, a) {
   |        --------------
   |        |           |
   |        |           matches any value
   |        in this macro invocation
   |
note: the lint level is defined here
  --> src/main.rs:3:16
   |
3  |         #[deny(unreachable_patterns)]
   |                ^^^^^^^^^^^^^^^^^^^^
...
17 |     if matches!(1, a) {
   |        -------------- in this macro invocation
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
