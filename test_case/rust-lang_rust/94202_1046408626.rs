plain
[RUSTC-TIMING] rustc_attr test:false 6.108
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_errors test:false 18.292
[RUSTC-TIMING] rustc_query_system test:false 11.187
error: passing `ty::consts::Const<'tcx>` by reference
  --> compiler/rustc_middle/src/ty/consts.rs:51:16
   |
51 |     pub fn val(&self) -> &ConstKind<'tcx> {
   |                ^^^^^ help: try passing by value: `ty::consts::Const<'tcx>`
   |
   = note: `-D rustc::pass-by-value` implied by `-D warnings`
[RUSTC-TIMING] rustc_middle test:false 18.816
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_session test:false 27.445
