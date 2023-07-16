plain
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0433]: failed to resolve: could not find `blocks` in `map`
     |
     |
1059 |         if let Some(fn_node) = rustc_middle::hir::map::blocks::FnLikeNode::from_node(node) {
     |                                                        ^^^^^^ could not find `blocks` in `map`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
