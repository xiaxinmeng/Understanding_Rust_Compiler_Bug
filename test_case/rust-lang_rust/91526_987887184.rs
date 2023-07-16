plain
    Checking clippy_lints v0.1.59 (/checkout/src/tools/clippy/clippy_lints)
error: unused import: `LintContext`
  --> src/tools/clippy/clippy_lints/src/index_refutable_slice.rs:11:45
   |
11 | use rustc_lint::{LateContext, LateLintPass, LintContext};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `LintContext`
  --> src/tools/clippy/clippy_lints/src/matches.rs:27:45
   |
   |
27 | use rustc_lint::{LateContext, LateLintPass, LintContext};


error[E0609]: no field `sess` on type `&EarlyContext<'tcx>`
  --> src/tools/clippy/clippy_lints/src/octal_escapes.rs:54:33
   |
54 |         if in_external_macro(cx.sess, expr.span) {
   |
   |
   = note: available fields are: `builder`, `buffered`
   |
   |
54 |         if in_external_macro(cx.builder.sess, expr.span) {

For more information about this error, try `rustc --explain E0609`.
error: could not compile `clippy_lints` due to 3 previous errors
Build completed unsuccessfully in 0:04:11
