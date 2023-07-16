plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0658]: negative trait bounds are not yet fully implemented; use marker types for now
  --> compiler/rustc_hir/src/hir_id.rs:24:6
   |
24 | impl !Ord for HirId {}
   |
   = note: see issue #68318 <https://github.com/rust-lang/rust/issues/68318> for more information
   = help: add `#![feature(negative_impls)]` to the crate attributes to enable


error[E0658]: negative trait bounds are not yet fully implemented; use marker types for now
  --> compiler/rustc_hir/src/hir_id.rs:25:6
   |
25 | impl !PartialOrd for HirId {}
   |
   = note: see issue #68318 <https://github.com/rust-lang/rust/issues/68318> for more information
   = help: add `#![feature(negative_impls)]` to the crate attributes to enable

