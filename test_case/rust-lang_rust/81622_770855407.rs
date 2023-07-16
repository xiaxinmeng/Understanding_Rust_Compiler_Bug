plain
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error[E0308]: mismatched types
   --> compiler/rustc_privacy/src/lib.rs:136:13
    |
135 |         const_evaluatable::walk_abstract_const(tcx, ct, |node| match node {
    |                                                                      ---- this expression has type `AbstractConst<'_>`
136 |             ACNode::Leaf(leaf) => {
    |             ^^^^^^^^^^^^^^^^^^ expected struct `AbstractConst`, found enum `rustc_middle::mir::abstract_const::Node`
error[E0308]: mismatched types
   --> compiler/rustc_privacy/src/lib.rs:140:13
    |
    |
135 |         const_evaluatable::walk_abstract_const(tcx, ct, |node| match node {
    |                                                                      ---- this expression has type `AbstractConst<'_>`
...
140 |             ACNode::Binop(..) | ACNode::UnaryOp(..) | ACNode::FunctionCall(_, _) => {
    |             ^^^^^^^^^^^^^^^^^ expected struct `AbstractConst`, found enum `rustc_middle::mir::abstract_const::Node`
error[E0308]: mismatched types
   --> compiler/rustc_privacy/src/lib.rs:140:33
    |
    |
135 |         const_evaluatable::walk_abstract_const(tcx, ct, |node| match node {
    |                                                                      ---- this expression has type `AbstractConst<'_>`
...
140 |             ACNode::Binop(..) | ACNode::UnaryOp(..) | ACNode::FunctionCall(_, _) => {
    |                                 ^^^^^^^^^^^^^^^^^^^ expected struct `AbstractConst`, found enum `rustc_middle::mir::abstract_const::Node`
error[E0308]: mismatched types
   --> compiler/rustc_privacy/src/lib.rs:140:55
    |
    |
135 |         const_evaluatable::walk_abstract_const(tcx, ct, |node| match node {
    |                                                                      ---- this expression has type `AbstractConst<'_>`
...
140 |             ACNode::Binop(..) | ACNode::UnaryOp(..) | ACNode::FunctionCall(_, _) => {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `AbstractConst`, found enum `rustc_middle::mir::abstract_const::Node`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_privacy`
error: could not compile `rustc_privacy`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_arena" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_ast_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_attr" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_plugin_impl" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_feature" "-p" "rustc_ast" "-p" "rustc_hir_pretty" "-p" "rustc_target" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_metadata" "-p" "rustc_errors" "-p" "rustc_error_codes" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:31
