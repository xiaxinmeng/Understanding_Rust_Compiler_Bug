
 error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:43:35
   |
39 | / macro_rules! assert_eq {
40 | |     ($left:expr, $right:expr $(,)?) => ({
41 | |         match (&$left, &$right) {
42 | |             (left_val, right_val) => {
43 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected a tuple with 3 elements, found one with 2 elements
...  |
68 | |     });
69 | | }
   | |_- in this expansion of `assert_eq!`
   | 
  ::: src/librustdoc/doctest/tests.rs:15:5
   |
15 |       assert_eq!(output, (expected, 2));
   |       ---------------------------------- in this macro invocation
   |
   = note: expected tuple `(std::string::String, usize, bool)`
              found tuple `(std::string::String, {integer})`
