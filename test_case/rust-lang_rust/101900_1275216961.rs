plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.6.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.66 (/checkout/src/tools/clippy/clippy_lints)
error[E0277]: the trait bound `[closure@src/tools/clippy/tests/compile-test.rs:84:31: 84:33]: FnMut<()>` is not satisfied
   |
   |
98 |         if let Some((name, path)) = parse_name_path() {
   |                                     ^^^^^^^^^^^^^^^^^ expected an `FnMut<()>` closure, found `[closure@src/tools/clippy/tests/compile-test.rs:84:31: 84:33]`
   |
   = help: the trait `~const FnMut<()>` is not implemented for closure `[closure@src/tools/clippy/tests/compile-test.rs:84:31: 84:33]`
note: the trait `FnMut<()>` is implemented for `[closure@src/tools/clippy/tests/compile-test.rs:84:31: 84:33]`, but that implementation is not `const`
   |
   |
98 |         if let Some((name, path)) = parse_name_path() {
   |                                     ^^^^^^^^^^^^^^^^^
   = note: wrap the `[closure@src/tools/clippy/tests/compile-test.rs:84:31: 84:33]` in a closure with no arguments: `|| { /* code */ }`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `clippy` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:56
