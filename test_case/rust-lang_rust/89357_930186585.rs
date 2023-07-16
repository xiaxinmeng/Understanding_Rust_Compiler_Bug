plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:39:35
   |
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
...  |
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   | |_- in this expansion of `assert_eq!`
   |
  ::: library/core/tests/cell.rs:55:5
   |
55 |       assert_eq!(x.update(|x| x + 5), 15);

error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:39:35
   |
   |
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
...  |
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   | |_- in this expansion of `assert_eq!`
   |
  ::: library/core/tests/cell.rs:58:5
   |
58 |       assert_eq!(x.update(|x| x / 3), 5);

For more information about this error, try `rustc --explain E0308`.
error: could not compile `core` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
