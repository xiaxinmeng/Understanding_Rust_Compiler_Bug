plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0433]: failed to resolve: could not find `rustc_data_structures` in the list of imported crates
   --> compiler/rustc_attr/src/builtin.rs:139:10
    |
139 |   #[derive(HashStable_Generic)]
    |            |
    |            could not find `rustc_data_structures` in the list of imported crates
    |            in this macro invocation
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable_Generic)]`
error[E0433]: failed to resolve: could not find `rustc_data_structures` in the list of imported crates
   --> compiler/rustc_attr/src/builtin.rs:147:10
    |
    |
147 |   #[derive(HashStable_Generic)]
    |            |
    |            could not find `rustc_data_structures` in the list of imported crates
    |            in this macro invocation
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable_Generic)]`
error[E0433]: failed to resolve: could not find `rustc_data_structures` in the list of imported crates
   --> compiler/rustc_attr/src/builtin.rs:157:10
    |
    |
157 |   #[derive(HashStable_Generic)]
    |            |
    |            could not find `rustc_data_structures` in the list of imported crates
    |            in this macro invocation
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable_Generic)]`
error[E0433]: failed to resolve: could not find `rustc_data_structures` in the list of imported crates
   --> compiler/rustc_attr/src/builtin.rs:655:46
    |
    |
655 |   #[derive(Debug, Encodable, Decodable, Clone, HashStable_Generic)]
    |                                                |
    |                                                could not find `rustc_data_structures` in the list of imported crates
    |                                                in this macro invocation
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable_Generic)]`
error[E0433]: failed to resolve: could not find `rustc_data_structures` in the list of imported crates
   --> compiler/rustc_attr/src/builtin.rs:831:32
    |
    |
831 |   #[derive(Encodable, Decodable, HashStable_Generic)]
    |                                  |
    |                                  could not find `rustc_data_structures` in the list of imported crates
    |                                  in this macro invocation
    | 
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable_Generic)]`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_attr`
error: could not compile `rustc_attr`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_error_codes" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_fs_util" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_ast_passes" "-p" "rustc_traits" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_ast_lowering" "-p" "rustc_arena" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_passes" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_typeck" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_attr" "-p" "rustc_privacy" "-p" "rustc_lint" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_plugin_impl" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_session" "-p" "rustc_target" "-p" "rustc_ast_pretty" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_metadata" "-p" "rustc_save_analysis" "-p" "rustc_serialize" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:50
