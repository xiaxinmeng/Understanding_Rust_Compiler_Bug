plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0599]: no variant or associated item named `Zst` found for enum `rustc_middle::mir::interpret::ConstValue` in the current scope
   --> compiler/rustc_const_eval/src/const_eval/eval_queries.rs:168:53
    |
168 |             _ if imm.layout.is_zst() => ConstValue::Zst,
    |                                                     ^^^ variant or associated item not found in `rustc_middle::mir::interpret::ConstValue<'_>`
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_const_eval` due to previous error
