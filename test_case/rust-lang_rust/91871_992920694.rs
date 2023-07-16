plain
   Compiling once_cell v1.7.2
   Compiling difference v2.0.0
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: can't compare `&[&str]` with `url_parts_builder::UrlPartsBuilder`
   |
35 | / macro_rules! assert_eq {
35 | / macro_rules! assert_eq {
36 | |     ($left:expr, $right:expr $(,)?) => ({
37 | |         match (&$left, &$right) {
38 | |             (left_val, right_val) => {
39 | |                 if !(*left_val == *right_val) {
   | |                                ^^ no implementation for `&[&str] == url_parts_builder::UrlPartsBuilder`
61 | |     });
62 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: src/librustdoc/html/tests.rs:6:5
   |
6  |       assert_eq!(expected, href_relative_parts(&fqp, &relative_to_fqp));
   |
   |
   = help: the trait `std::cmp::PartialEq<url_parts_builder::UrlPartsBuilder>` is not implemented for `&[&str]`

error[E0599]: the method `collect` exists for struct `Vec<&str>`, but its trait bounds were not satisfied
   --> src/librustdoc/html/url_parts_builder/tests.rs:45:27
    |
45  |     t(vec!["core", "str"].collect(), "core/str");
    |
   ::: /checkout/library/alloc/src/vec/mod.rs:400:1
    |
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    | ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<&str>: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `Vec<&str>: Iterator`
            which is required by `&mut Vec<&str>: Iterator`
            `[&str]: Iterator`
            which is required by `&mut [&str]: Iterator`

error[E0599]: the method `collect` exists for struct `Vec<&str>`, but its trait bounds were not satisfied
   --> src/librustdoc/html/url_parts_builder/tests.rs:46:48
    |
46  |     t(vec!["core", "str", "struct.Bytes.html"].collect(), "core/str/struct.Bytes.html");
    |
   ::: /checkout/library/alloc/src/vec/mod.rs:400:1
    |
    |
400 | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    | ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<&str>: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `Vec<&str>: Iterator`
            which is required by `&mut Vec<&str>: Iterator`
            `[&str]: Iterator`
            which is required by `&mut [&str]: Iterator`

error[E0271]: type mismatch resolving `<&[&str; 2] as IntoIterator>::Item == &str`
   --> src/librustdoc/html/url_parts_builder/tests.rs:52:13
    |
52  |     builder.extend(&["str", "struct.Bytes.html"]);
    |             ^^^^^^ expected `str`, found `&str`
    = note: expected reference `&str`
               found reference `&&str`
note: required by a bound in `std::iter::Extend::extend`
   --> /checkout/library/core/src/iter/traits/collect.rs:339:31
   --> /checkout/library/core/src/iter/traits/collect.rs:339:31
    |
339 |     fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
    |                               ^^^^^^^^ required by this bound in `std::iter::Extend::extend`
Some errors have detailed explanations: E0271, E0277, E0599.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `rustdoc` due to 4 previous errors



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:29:10
