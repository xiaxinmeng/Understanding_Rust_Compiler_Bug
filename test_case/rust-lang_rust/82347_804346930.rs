plain
[RUSTC-TIMING] semver test:false 1.034
   Compiling cargo_metadata v0.11.1
[RUSTC-TIMING] serde_json test:false 2.798
   Compiling tidy v0.1.0 (D:\a\rust\rust\src\tools\tidy)
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> src\tools\tidy\src\bins.rs:15:47
   |
15 |     pub fn check_filesystem_support(_sources: &[Path], _output: &Path) -> bool {
   |                                               ^^^^^^^ doesn't have a size known at compile-time
   |
   = help: within `Path`, the trait `Sized` is not implemented for `[u8]`
   = note: required because it appears within the type `Path`
   = note: slice and array elements must have `Sized` type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] tidy test:false 0.290
---
command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\rust\\rust\\src/tools/tidy\\Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:00:33
make: *** [Makefile:76: ci-subset-1] Error 1
