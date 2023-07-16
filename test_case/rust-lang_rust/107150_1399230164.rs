plain
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
[RUSTC-TIMING] rustc_session test:false 6.051
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0616]: field `gcx` of struct `context::TyCtxt` is private
   --> compiler/rustc_middle/src/ty/tls.rs:159:25
    |
159 |             context.tcx.gcx as *const _ as *const (),


error[E0616]: field `gcx` of struct `context::TyCtxt` is private
   --> compiler/rustc_middle/src/ty/tls.rs:160:17
    |
160 |             tcx.gcx as *const _ as *const ()

[RUSTC-TIMING] rustc_ast_passes test:false 4.542
For more information about this error, try `rustc --explain E0616`.
[RUSTC-TIMING] rustc_middle test:false 8.943
