plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_serialize::opaque::Encoder: OpaqueEncoder` is not satisfied
    --> compiler/rustc_middle/src/ty/query/on_disk_cache.rs:1182:16
     |
1182 | impl<'a, 'tcx> Encodable<CacheEncoder<'a, 'tcx, opaque::Encoder>> for [u8] {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `OpaqueEncoder` is not implemented for `rustc_serialize::opaque::Encoder`
    ::: /checkout/compiler/rustc_serialize/src/serialize.rs:393:24
     |
     |
393  | pub trait Encodable<S: Encoder> {
     |                        ------- required by this bound in `Encodable`
     |
     = note: required because of the requirements on the impl of `rustc_serialize::Encoder` for `CacheEncoder<'a, 'tcx, rustc_serialize::opaque::Encoder>`

error[E0277]: the trait bound `rustc_serialize::opaque::Encoder: OpaqueEncoder` is not satisfied
    --> compiler/rustc_middle/src/ty/query/on_disk_cache.rs:1183:25
     |
984  | struct CacheEncoder<'a, 'tcx, E: OpaqueEncoder> {
     |                                  ------------- required by this bound in `CacheEncoder`
...
1183 |     fn encode(&self, e: &mut CacheEncoder<'a, 'tcx, opaque::Encoder>) -> opaque::EncodeResult {
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `OpaqueEncoder` is not implemented for `rustc_serialize::opaque::Encoder`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_macros" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "rustc_attr" "-p" "rustc_apfloat" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_index" "-p" "coverage_test_macros" "-p" "rustc_graphviz" "-p" "rustc_interface" "-p" "rustc_symbol_mangling" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_mir_build" "-p" "rustc_arena" "-p" "rustc_typeck" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_ast_passes" "-p" "rustc_incremental" "-p" "rustc_expand" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_data_structures" "-p" "rustc_error_codes" "-p" "rustc_session" "-p" "rustc_parse" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_ast" "-p" "rustc_hir_pretty" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_metadata" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_lint" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:55
