plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `Empty` in this scope
   |
   |
69 |                 let mut dependency_of = Empty;
   |
help: consider importing one of these items
   |
10 | use core::num::IntErrorKind::Empty;
10 | use core::num::IntErrorKind::Empty;
   |
10 | use rustc_ast::MacArgs::Empty;
   |
10 | use rustc_ast::StmtKind::Empty;
10 | use rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault::Empty;
   |
     and 6 other candidates


error[E0425]: cannot find value `Empty` in this scope
   |
   |
83 |                 let mut orig_crate_name = Empty;
   |
help: consider importing one of these items
   |
10 | use core::num::IntErrorKind::Empty;
10 | use core::num::IntErrorKind::Empty;
   |
10 | use rustc_ast::MacArgs::Empty;
   |
10 | use rustc_ast::StmtKind::Empty;
10 | use rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault::Empty;
   |
     and 6 other candidates


error[E0425]: cannot find value `Empty` in this scope
   |
   |
84 |                 let mut orig_dependency_of = Empty;
   |
help: consider importing one of these items
   |
10 | use core::num::IntErrorKind::Empty;
10 | use core::num::IntErrorKind::Empty;
   |
10 | use rustc_ast::MacArgs::Empty;
   |
10 | use rustc_ast::StmtKind::Empty;
10 | use rustc_middle::middle::resolve_lifetime::ObjectLifetimeDefault::Empty;
   |
     and 6 other candidates


error: unused import: `Symbol`
  --> compiler/rustc_passes/src/lang_items.rs:23:24
   |
23 | use rustc_span::{Span, Symbol};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_passes` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_passes` due to 4 previous errors
