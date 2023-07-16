plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0277]: can't compare `LintExpectationId` with `LintExpectationId`
     |
     |
150  |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
...
...
165  |       Expect(LintExpectationId),
     |              ^^^^^^^^^^^^^^^^^ no implementation for `LintExpectationId < LintExpectationId` and `LintExpectationId > LintExpectationId`
    ::: /checkout/library/core/src/cmp.rs:1172:1
     |
1172 | / pub macro PartialOrd($item:item) {
1173 | |     /* compiler built-in */
1173 | |     /* compiler built-in */
1174 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `LintExpectationId`

error[E0277]: the trait bound `LintExpectationId: Ord` is not satisfied
    |
    |
150 |   #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
...
...
165 |       Expect(LintExpectationId),
    |              ^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `LintExpectationId`
   ::: /checkout/library/core/src/cmp.rs:860:1
    |
    |
860 | / pub macro Ord($item:item) {
862 | | }
862 | | }
    | |_- in this expansion of `#[derive(Ord)]`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_lint_defs` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
