plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0412]: cannot find type `HirId` in this scope
    --> compiler/rustc_ast_lowering/src/expr.rs:1931:17
     |
1931 |         hir_id: HirId,
     |
help: consider importing one of these items
     |
     |
1    | use crate::hir::HirId;
1    | use rustc_hir::HirId;
     |

For more information about this error, try `rustc --explain E0412`.
