plain
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error: expected one of `+`, `,`, or `::`, found `{`
    --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1694:76
     |
1694 |         let find_param_matching = |matches: &dyn Fn(&ty::ParamTy) -> bool| {
     |                                                                            ^ expected one of `+`, `,`, or `::`
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:09:35
