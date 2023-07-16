plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error: unused import: `std::env`
  --> compiler/rustc_query_system/src/dep_graph/graph.rs:16:5
16 | use std::env;
   |     ^^^^^^^^
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: unused import: `super::debug::EdgeFilter`
  --> compiler/rustc_query_system/src/dep_graph/graph.rs:22:5
22 | use super::debug::EdgeFilter;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: try expression alternatives have incompatible types
error[E0308]: try expression alternatives have incompatible types
   --> compiler/rustc_query_system/src/dep_graph/serialized.rs:255:27
    |
255 |       let node_count: u64 = ordered_recv(recv, |_index, node| {
256 | |         #[cfg(debug_assertions)]
256 | |         #[cfg(debug_assertions)]
257 | |         if let Some(record_graph) = &_record_graph {
258 | |             record_graph.lock().unwrap().push(_index, node.node, &node.edges);
...   |
276 | |         node.encode(&mut encoder)
277 | |     })?;
    | |_______^ expected `u64`, found `u32`
    |
help: you can convert a `u32` to a `u64`
    |
255 |     let node_count: u64 = ordered_recv(recv, |_index, node| {
256 |         #[cfg(debug_assertions)]
257 |         if let Some(record_graph) = &_record_graph {
258 |             record_graph.lock().unwrap().push(_index, node.node, &node.edges);
260 | 
  ...

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_query_system`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_feature" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_ast" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_target" "-p" "rustc_symbol_mangling" "-p" "rustc_apfloat" "-p" "rustc_span" "-p" "rustc_macros" "-p" "rustc_incremental" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_index" "-p" "rustc_errors" "-p" "rustc_driver" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:14
