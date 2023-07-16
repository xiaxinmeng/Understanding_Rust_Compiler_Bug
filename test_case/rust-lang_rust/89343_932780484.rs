plain
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
error[E0600]: cannot apply unary operator `!` to type `FingerprintStyle`
  --> compiler/rustc_query_system/src/dep_graph/dep_node.rs:78:16
   |
78 |             if !kind.fingerprint_style()
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^ cannot apply unary operator `!`
   |
   = note: an implementation of `std::ops::Not` might be missing for `FingerprintStyle`
For more information about this error, try `rustc --explain E0600`.
error: could not compile `rustc_query_system` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
