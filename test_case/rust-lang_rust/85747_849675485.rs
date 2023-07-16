plain
---- src/fs.rs - fs::Metadata::is_symlink (line 1014) stdout ----
error[E0658]: use of unstable library feature 'is_symlink'
  --> src/fs.rs:1025:22
   |
13 |     assert!(metadata.is_symlink());
   |
   = note: see issue #85748 <https://github.com/rust-lang/rust/issues/85748> for more information
   = note: see issue #85748 <https://github.com/rust-lang/rust/issues/85748> for more information
   = help: add `#![feature(is_symlink)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/path.rs - path::Path::is_symlink (line 2582) stdout ----
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
   |
   |
3  |   fn main() { #[allow(non_snake_case)] fn _doctest_main_src_path_rs_2582_0() {
4  | | use std::path::Path;
5  | | use std::os::unix::fs::symlink;
6  | |
6  | |
7  | | let link_path = Path::new("/link");
8  | | symlink("/origin_does_not_exists/", link_path)?;
   | |                                               ^ cannot use the `?` operator in a function that returns `()`
9  | | assert_eq!(link_path.is_symlink(), true);
10 | | assert_eq!(link_path.exists(), false);
11 | | } _doctest_main_src_path_rs_2582_0() }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
   = note: required by `from_residual`
error[E0658]: use of unstable library feature 'is_symlink'
 --> src/path.rs:2588:22
  |
  |
9 | assert_eq!(link_path.is_symlink(), true);
  |
  = note: see issue #85748 <https://github.com/rust-lang/rust/issues/85748> for more information
  = note: see issue #85748 <https://github.com/rust-lang/rust/issues/85748> for more information
  = help: add `#![feature(is_symlink)]` to the crate attributes to enable
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0658.
For more information about an error, try `rustc --explain E0277`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:17
