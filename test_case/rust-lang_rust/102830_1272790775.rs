plain
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0599]: no variant or associated item named `TyAlias` found for enum `ImplItemKind` in the current scope
  --> compiler/rustc_const_eval/src/const_eval/fn_queries.rs:67:38
   |
67 |             kind: hir::ImplItemKind::TyAlias(..) | hir::ImplItemKind::Fn(..),
   |                                      ^^^^^^^ variant or associated item not found in `ImplItemKind<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_const_eval` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_const_eval` due to previous error
