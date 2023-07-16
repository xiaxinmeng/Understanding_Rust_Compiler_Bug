plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error[E0277]: can't compare `&mut [{integer}; 0]` with `[_; 0]`
     |
35   | / macro_rules! assert_eq {
35   | / macro_rules! assert_eq {
36   | |     ($left:expr, $right:expr $(,)?) => ({
37   | |         match (&$left, &$right) {
38   | |             (left_val, right_val) => {
39   | |                 if !(*left_val == *right_val) {
     | |                                ^^ no implementation for `&mut [{integer}; 0] == [_; 0]`
61   | |     });
62   | | }
     | |_- in this expansion of `assert_eq!`
     |
     |
    ::: library/core/tests/slice.rs:2229:9
     |
2229 |           assert_eq!(right, []);
     |
     |
     = help: the trait `PartialEq<[_; 0]>` is not implemented for `&mut [{integer}; 0]`

error[E0277]: can't compare `&mut [{integer}; 6]` with `[{integer}; 6]`
     |
35   | / macro_rules! assert_eq {
35   | / macro_rules! assert_eq {
36   | |     ($left:expr, $right:expr $(,)?) => ({
37   | |         match (&$left, &$right) {
38   | |             (left_val, right_val) => {
39   | |                 if !(*left_val == *right_val) {
     | |                                ^^ no implementation for `&mut [{integer}; 6] == [{integer}; 6]`
61   | |     });
62   | | }
     | |_- in this expansion of `assert_eq!`
     |
     |
    ::: library/core/tests/slice.rs:2235:9
     |
2235 |           assert_eq!(right, [1, 2, 3, 4, 5, 6]);
     |
     |
     = help: the trait `PartialEq<[{integer}; 6]>` is not implemented for `&mut [{integer}; 6]`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
