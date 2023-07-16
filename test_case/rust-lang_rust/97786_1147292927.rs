plain

---- [ui] src/test/ui/span/issue-71363.rs stdout ----
diff of stderr:

1 error[E0277]: `MyError` doesn't implement `std::fmt::Display`
-   |
-   |
- 6 | impl std::error::Error for MyError {}
-   |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted with the default formatter
-   |
-   = help: the trait `std::fmt::Display` is not implemented for `MyError`
-   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
+     |
+     |
+ 6   | impl std::error::Error for MyError {}
+     |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted with the default formatter
+     |
+     = help: the trait `std::fmt::Display` is not implemented for `MyError`
+     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
9 note: required by a bound in `std::error::Error`
+    --> $SRC_DIR/std/src/error.rs:LL:COL
+     |
+ 193 | pub trait Error: Debug + Display {
+     |                          ^^^^^^^ required by this bound in `std::error::Error`
10 
11 error[E0277]: `MyError` doesn't implement `Debug`
-   |
-   |
- 6 | impl std::error::Error for MyError {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted using `{:?}`
-   |
-   = help: the trait `Debug` is not implemented for `MyError`
-   = note: add `#[derive(Debug)]` to `MyError` or manually `impl Debug for MyError`
+     |
+     |
+ 6   | impl std::error::Error for MyError {}
+     |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted using `{:?}`
+     = help: the trait `Debug` is not implemented for `MyError`
+     = help: the trait `Debug` is not implemented for `MyError`
+     = note: add `#[derive(Debug)]` to `MyError` or manually `impl Debug for MyError`
19 note: required by a bound in `std::error::Error`
+    --> $SRC_DIR/std/src/error.rs:LL:COL
+     |
+ 193 | pub trait Error: Debug + Display {
+     |                  ^^^^^ required by this bound in `std::error::Error`
20 help: consider annotating `MyError` with `#[derive(Debug)]`
-   |
- 5 | #[derive(Debug)]
-   |
+ 5   | #[derive(Debug)]
+     |
24 
25 error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args span/issue-71363.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-71363.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "simulate-remapped-rust-src-base=/rustc/xyz" "-Z" "ui-testing=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `MyError` doesn't implement `std::fmt::Display`
    |
    |
6   | impl std::error::Error for MyError {}
    |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted with the default formatter
    |
    = help: the trait `std::fmt::Display` is not implemented for `MyError`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `std::error::Error`
    |
    |
193 | pub trait Error: Debug + Display {
    |                          ^^^^^^^ required by this bound in `std::error::Error`

error[E0277]: `MyError` doesn't implement `Debug`
    |
    |
6   | impl std::error::Error for MyError {}
    |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted using `{:?}`
    = help: the trait `Debug` is not implemented for `MyError`
    = help: the trait `Debug` is not implemented for `MyError`
    = note: add `#[derive(Debug)]` to `MyError` or manually `impl Debug for MyError`
note: required by a bound in `std::error::Error`
    |
    |
193 | pub trait Error: Debug + Display {
    |                  ^^^^^ required by this bound in `std::error::Error`
help: consider annotating `MyError` with `#[derive(Debug)]`
5   | #[derive(Debug)]
    |

error: aborting due to 2 previous errors
