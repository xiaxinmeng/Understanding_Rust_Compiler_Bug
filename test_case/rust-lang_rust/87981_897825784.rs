plain
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error: unused imports: `StmtKind`, `Stmt`
  --> compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs:11:49
   |
11 | use rustc_hir::{Expr, ExprKind, ItemKind, Node, Stmt, StmtKind};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:10:29
