plain
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error[E0433]: failed to resolve: use of undeclared type `FnKind`
   --> compiler/rustc_ast_passes/src/errors.rs:630:22
    |
630 |             Self::Fn(FnKind::Closure(..)) => {
    |                      ^^^^^^ use of undeclared type `FnKind`
error[E0433]: failed to resolve: use of undeclared type `FnKind`
   --> compiler/rustc_ast_passes/src/errors.rs:633:22
    |
    |
633 |             Self::Fn(FnKind::Fn(_, ident, ..)) => {
    |                      ^^^^^^ use of undeclared type `FnKind`

error[E0412]: cannot find type `DisallowTildeConstContext` in this scope
    |
    |
615 |     pub reason: DisallowTildeConstContext,
    |
    |
note: enum `crate::ast_validation::DisallowTildeConstContext` exists but is inaccessible
   --> compiler/rustc_ast_passes/src/ast_validation.rs:39:1
    |
39  | enum DisallowTildeConstContext<'a> {


error[E0412]: cannot find type `DisallowTildeConstContext` in this scope
    |
    |
618 | impl AddToDiagnostic for DisallowTildeConstContext {
    |
    |
note: enum `crate::ast_validation::DisallowTildeConstContext` exists but is inaccessible
   --> compiler/rustc_ast_passes/src/ast_validation.rs:39:1
    |
39  | enum DisallowTildeConstContext<'a> {

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_ast_passes` due to 4 previous errors
