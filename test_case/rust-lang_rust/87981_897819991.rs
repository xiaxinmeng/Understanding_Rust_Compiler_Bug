plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error: unused imports: `StmtKind`, `Stmt`
  --> compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs:11:49
   |
11 | use rustc_hir::{Expr, ExprKind, ItemKind, Node, Stmt, StmtKind};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
