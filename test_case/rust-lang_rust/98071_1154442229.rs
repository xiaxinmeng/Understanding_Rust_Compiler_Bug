plain
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error: usage of `ty::TyKind::<kind>`
     |
     |
1333 |             TyKind::Array(_, len) => len.try_eval_usize(tcx, param_env),
     |             ^^^^^^ help: try using `ty::<kind>` directly: `ty`
     |
     = note: `-D rustc::usage-of-ty-tykind` implied by `-D warnings`
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:09:24
