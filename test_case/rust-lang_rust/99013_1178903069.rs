plain
[RUSTC-TIMING] rustc_ast_lowering test:false 23.808
   Compiling rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
[RUSTC-TIMING] rustc_passes test:false 24.202
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error[E0599]: no variant or associated item named `Zst` found for enum `rustc_middle::mir::interpret::ConstValue` in the current scope
   --> compiler/rustc_const_eval/src/const_eval/eval_queries.rs:168:53
    |
168 |             _ if imm.layout.is_zst() => ConstValue::Zst,
    |                                                     ^^^ variant or associated item not found in `rustc_middle::mir::interpret::ConstValue<'_>`
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] rustc_const_eval test:false 2.045
error: could not compile `rustc_const_eval` due to previous error
warning: build failed, waiting for other jobs to finish...
