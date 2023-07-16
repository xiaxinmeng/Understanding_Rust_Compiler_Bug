plain
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: using `iter` can result in unstable query results
    --> compiler/rustc_mir_pretty/src/pretty.rs:1025:26
     |
1025 |         tcx.mir_keys(()).iter().map(|def_id| def_id.to_def_id()).collect()
     |
     |
     = note: `-D rustc::potential-query-instability` implied by `-D warnings`
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: could not compile `rustc_mir_pretty` due to previous error
error: build failed
Build completed unsuccessfully in 0:09:05
