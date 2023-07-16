plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling tracing v0.1.25
   Compiling tracing-subscriber v0.2.16
   Compiling rustfix v0.6.0
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:5:24
     |
5    |     assert_eq!(TestCx::normalize_platform_differences(r"$DIR\foo.rs"), "$DIR/foo.rs");
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:7:17
     |
7    |         TestCx::normalize_platform_differences(r"$BUILD_DIR\..\parser.rs"),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:11:17
     |
11   |         TestCx::normalize_platform_differences(r"$DIR\bar.rs hello\nworld"),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:15:17
     |
15   |         TestCx::normalize_platform_differences(r"either bar\baz.rs or bar\baz\mod.rs"),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:18:24
     |
18   |     assert_eq!(TestCx::normalize_platform_differences(r"`.\some\path.rs`"), r"`./some/path.rs`",);
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:19:24
     |
19   |     assert_eq!(TestCx::normalize_platform_differences(r"`some\path.rs`"), r"`some/path.rs`",);
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:21:17
     |
21   |         TestCx::normalize_platform_differences(r"$DIR\path-with-dashes.rs"),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:25:17
     |
25   |         TestCx::normalize_platform_differences(r"$DIR\path_with_underscores.rs"),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:28:24
     |
28   |     assert_eq!(TestCx::normalize_platform_differences(r"$DIR\foo.rs:12:11"), "$DIR/foo.rs:12:11",);
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:30:17
     |
30   |         TestCx::normalize_platform_differences(r"$DIR\path with spaces 'n' quotes"),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:34:17
     |
34   |         TestCx::normalize_platform_differences(r"$DIR\file_with\no_extension"),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:38:24
     |
38   |     assert_eq!(TestCx::normalize_platform_differences(r"\n"), r"\n");
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:39:24
     |
39   |     assert_eq!(TestCx::normalize_platform_differences(r"{ \n"), r"{ \n");
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:40:24
     |
40   |     assert_eq!(TestCx::normalize_platform_differences(r"`\]`"), r"`\]`");
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:41:24
     |
41   |     assert_eq!(TestCx::normalize_platform_differences(r#""\{""#), r#""\{""#);
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:43:17
     |
43   |         TestCx::normalize_platform_differences(r#"write!(&mut v, "Hello\n")"#),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
2069 |     fn normalize_platform_differences(output: &str) -> String {
     |     --------------------------------------------------------- private associated function defined here

error[E0624]: associated function `normalize_platform_differences` is private
    --> src/tools/compiletest/src/runtest/tests.rs:47:17
     |
47   |         TestCx::normalize_platform_differences(r#"println!("test\ntest")"#),
     |
    ::: src/tools/compiletest/src/runtest/test_cx.rs:2069:5
     |
2069 |     fn normalize_platform_differences(output: &str) -> String {
