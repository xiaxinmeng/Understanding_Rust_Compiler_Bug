plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error[E0277]: can't compare `borrow::Cow<'_, str>` with `std::borrow::Cow<'_, str>`
   |
36 | / macro_rules! assert_eq {
36 | / macro_rules! assert_eq {
37 | |     ($left:expr, $right:expr $(,)?) => {
38 | |         match (&$left, &$right) {
39 | |             (left_val, right_val) => {
40 | |                 if !(*left_val == *right_val) {
   | |                                ^^ no implementation for `borrow::Cow<'_, str> == std::borrow::Cow<'_, str>`
62 | |     };
63 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: library/alloc/src/ffi/c_str/tests.rs:57:9
   |
57 |           assert_eq!(CStr::from_ptr(ptr).to_string_lossy(), Borrowed("123…"));
   |
   |
   = help: the trait `PartialEq<std::borrow::Cow<'_, str>>` is not implemented for `borrow::Cow<'_, str>`

error[E0277]: can't compare `borrow::Cow<'_, str>` with `std::borrow::Cow<'_, str>`
   |
36 |  / macro_rules! assert_eq {
36 |  / macro_rules! assert_eq {
37 |  |     ($left:expr, $right:expr $(,)?) => {
38 |  |         match (&$left, &$right) {
39 |  |             (left_val, right_val) => {
40 |  |                 if !(*left_val == *right_val) {
   |  |                                ^^ no implementation for `borrow::Cow<'_, str> == std::borrow::Cow<'_, str>`
62 |  |     };
63 |  | }
   |  |_- in this expansion of `assert_eq!`
   |
   |
  ::: library/alloc/src/ffi/c_str/tests.rs:63:9
   |
63 | /          assert_eq!(
64 | |              CStr::from_ptr(ptr).to_string_lossy(),
65 | |              Owned::<str>(std::format!("123\u{FFFD}"))
   | |__________- in this macro invocation
   |
   |
   = help: the trait `PartialEq<std::borrow::Cow<'_, str>>` is not implemented for `borrow::Cow<'_, str>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
