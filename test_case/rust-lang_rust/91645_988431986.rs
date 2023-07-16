plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error[E0277]: can't compare `{integer}` with `({integer},)`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                                ^^ no implementation for `{integer} == ({integer},)`
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: library/core/tests/future.rs:36:9
   |
36 |           assert_eq!(x, (0,));
   |
   |
   = help: the trait `PartialEq<({integer},)>` is not implemented for `{integer}`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
