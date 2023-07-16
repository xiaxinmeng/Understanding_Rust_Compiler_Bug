plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0004]: non-exhaustive patterns: `rustc_middle::thir::ExprKind::OffsetOf { .. }` not covered
    |
305 |         match expr.kind {
305 |         match expr.kind {
    |               ^^^^^^^^^ pattern `rustc_middle::thir::ExprKind::OffsetOf { .. }` not covered
note: `rustc_middle::thir::ExprKind<'_>` defined here
   --> /checkout/compiler/rustc_middle/src/thir.rs:462:5
    |
235 | pub enum ExprKind<'tcx> {
235 | pub enum ExprKind<'tcx> {
    | -----------------------
...
462 |     OffsetOf {
    |     ^^^^^^^^ not covered
    = note: the matched value is of type `rustc_middle::thir::ExprKind<'_>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
352 ~             | thir::ExprKind::Yield { .. } => false,
353 ~             rustc_middle::thir::ExprKind::OffsetOf { .. } => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_ty_utils` due to previous error
warning: build failed, waiting for other jobs to finish...
