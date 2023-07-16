plain
[RUSTC-TIMING] rustc_target test:false 32.249
[RUSTC-TIMING] rustc_session test:false 27.247
   Compiling rustc_infer v0.0.0 (/checkout/compiler/rustc_infer)
[RUSTC-TIMING] rustc_ast_passes test:false 18.337
   Compiling rustc_mir_pretty v0.0.0 (/checkout/compiler/rustc_mir_pretty)
error: using `iter` can result in unstable query results
    --> compiler/rustc_mir_pretty/src/pretty.rs:1025:26
     |
1025 |         tcx.mir_keys(()).iter().map(|def_id| def_id.to_def_id()).collect()
     |
     |
     = note: `-D rustc::potential-query-instability` implied by `-D warnings`
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
[RUSTC-TIMING] rustc_mir_pretty test:false 0.728
[RUSTC-TIMING] rustc_mir_pretty test:false 0.728
error: could not compile `rustc_mir_pretty` due to previous error
[RUSTC-TIMING] rustc_ast_lowering test:false 43.194
[RUSTC-TIMING] rustc_builtin_macros test:false 41.037
[RUSTC-TIMING] rustc_expand test:false 50.041
[RUSTC-TIMING] rustc_parse test:false 62.425
