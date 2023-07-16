plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `ItemSortKey<'_>: Ord` is not satisfied
   --> compiler/rustc_middle/src/mir/mono.rs:393:34
    |
393 |         items.sort_by_cached_key(|&(i, _)| item_sort_key(tcx, i));
    |               ------------------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `ItemSortKey<'_>`
    |               required by a bound introduced by this call

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to previous error
