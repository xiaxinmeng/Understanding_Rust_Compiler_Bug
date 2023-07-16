
-   83|       |//
-   84|       |// Even though this function is used by `uses_crate.rs` (and
-   85|       |// counted), with substitutions for `T`, those instantiations are only generated
-   86|       |// when the generic function is actually used (from the binary, not from this
-   87|       |// library crate). So the test result shows coverage for all instantiated
-   88|       |// versions and their generic type substitutions, plus the `Unexecuted
-   89|       |// instantiation` message for the non-substituted version. This is valid, but
-   90|       |// unfortunately a little confusing.
-   91|       |//
-   92|       |// The library crate has its own coverage map, and the only way to show unused
-   93|       |// coverage of a generic function is to include the generic function in the
-   94|       |// coverage map, marked as an "unused function". If the library were used by
-   95|       |// another binary that never used this generic function, then it would be valid
-   96|       |// to show the unused generic, with unknown substitution (`_`).
-   97|       |//
-   98|       |// The alternative is to exclude all generics from being included in the "unused
-   99|       |// functions" list, which would then omit coverage results for
-  100|       |// `unused_generic_function<T>()`, below.
------------------------------------------
--- stderr -------------------------------
warning: function is never used: `unused_private_function`
  --> ../coverage/lib/used_inline_crate.rs:74:4
---

Error: 1
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `3`: the argument was wrong', ../coverage/assert.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: function is never used: `do_not_add_coverage_not_called`
  --> ../coverage/no_cov_crate.rs:15:4
   |
15 | fn do_not_add_coverage_not_called() {
   |
   = note: `#[warn(dead_code)]` on by default

warning: function is never used: `add_coverage_not_called`
warning: function is never used: `add_coverage_not_called`
  --> ../coverage/no_cov_crate.rs:27:4
27 | fn add_coverage_not_called() {
   |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: 2 warnings emitted
warning: 2 warnings emitted

warning: unreachable statement
  --> ../coverage/issue-93054.rs:12:9
   |
11 |         match self { }
   |         -------------- any code following this expression is unreachable
12 |         make().map(|never| match never { });
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
   = note: `#[warn(unreachable_code)]` on by default

warning: enum is never used: `Never`
 --> ../coverage/issue-93054.rs:7:6
 --> ../coverage/issue-93054.rs:7:6
  |
7 | enum Never { }
  |
  = note: `#[warn(dead_code)]` on by default

warning: associated function is never used: `foo`
---

warning: function is never used: `foo2`
  --> ../coverage/issue-93054.rs:20:10
   |
20 | async fn foo2(never: Never) {


warning: function is never used: `make`
  --> ../coverage/issue-93054.rs:24:4
   |
24 | fn make() -> Option<Never> {

warning: 6 warnings emitted


diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs
make: *** [Makefile:123: uses_crate] Error 1



failures:
