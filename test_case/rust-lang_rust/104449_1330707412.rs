plain

7   = help: the trait `std::fmt::Display` is not implemented for `MyError`
8   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
9 note: required by a bound in `std::error::Error`
- /rustc/xyz/library/core/src/error.rs:31:26: required by this bound in `std::error::Error`
+  --> /rustc/xyz/library/core/src/error.rs:31:26
+   = note: required by this bound in `std::error::Error`
11 
11 
12 error[E0277]: `MyError` doesn't implement `Debug`

18   = help: the trait `Debug` is not implemented for `MyError`
18   = help: the trait `Debug` is not implemented for `MyError`
19   = note: add `#[derive(Debug)]` to `MyError` or manually `impl Debug for MyError`
20 note: required by a bound in `std::error::Error`
- /rustc/xyz/library/core/src/error.rs:31:18: required by this bound in `std::error::Error`
+  --> /rustc/xyz/library/core/src/error.rs:31:18
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   = note: required by this bound in `std::error::Error`
+   = note: required by this bound in `std::error::Error`
22 help: consider annotating `MyError` with `#[derive(Debug)]`
24 3 | #[derive(Debug)]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363/issue-71363.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-71363.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-71363.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363/auxiliary" "-Z" "simulate-remapped-rust-src-base=/rustc/xyz" "-Z" "translate-remapped-path-to-local-path=no" "-Z" "ui-testing=no"
stdout: none
--- stderr -------------------------------
error[E0277]: `MyError` doesn't implement `std::fmt::Display`
  |
4 | impl std::error::Error for MyError {}
4 | impl std::error::Error for MyError {}
  |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted with the default formatter
  = help: the trait `std::fmt::Display` is not implemented for `MyError`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `std::error::Error`
note: required by a bound in `std::error::Error`
 --> /rustc/xyz/library/core/src/error.rs:31:26
  = note: required by this bound in `std::error::Error`


error[E0277]: `MyError` doesn't implement `Debug`
  |
4 | impl std::error::Error for MyError {}
4 | impl std::error::Error for MyError {}
  |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted using `{:?}`
  = help: the trait `Debug` is not implemented for `MyError`
  = help: the trait `Debug` is not implemented for `MyError`
  = note: add `#[derive(Debug)]` to `MyError` or manually `impl Debug for MyError`
note: required by a bound in `std::error::Error`
 --> /rustc/xyz/library/core/src/error.rs:31:18
  = note: required by this bound in `std::error::Error`
  = note: required by this bound in `std::error::Error`
help: consider annotating `MyError` with `#[derive(Debug)]`
3 | #[derive(Debug)]
  |

error: aborting due to 2 previous errors
