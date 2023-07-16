plain
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
    Checking url v2.1.1
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.53 (/checkout/src/tools/clippy/clippy_lints)
error[E0425]: cannot find function `is_min_const_fn` in module `rustc_mir::const_eval`
   --> src/tools/clippy/clippy_lints/src/missing_const_for_fn.rs:142:39
    |
142 |             if rustc_mir::const_eval::is_min_const_fn(cx.tcx, def_id.to_def_id()) {
    | 
   ::: /checkout/compiler/rustc_mir/src/const_eval/fn_queries.rs:11:1
    |
    |
11  | pub fn is_const_fn(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    | ---------------------------------------------------------- similarly named function `is_const_fn` defined here
help: a function with a similar name exists
    |
    |
142 |             if rustc_mir::const_eval::is_const_fn(cx.tcx, def_id.to_def_id()) {
help: consider importing one of these items
    |
    |
1   | use clippy_utils::qualify_min_const_fn::is_min_const_fn;
    |
1   | use crate::neg_cmp_op_on_partial_ord::clippy_utils::qualify_min_const_fn::is_min_const_fn;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
