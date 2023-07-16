plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking chalk-solve v0.55.0
error[E0432]: unresolved import `rustc_span::crate_disambiguator`
 --> compiler/rustc_hir/src/tests.rs:3:17
  |
3 | use rustc_span::crate_disambiguator::CrateDisambiguator;
  |                 ^^^^^^^^^^^^^^^^^^^ could not find `crate_disambiguator` in `rustc_span`
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> compiler/rustc_hir/src/tests.rs:26:31
    |
    |
26  |         let stable_crate_id = StableCrateId::new(crate_name, crate_disambiguator);
    |                               |
    |                               expected 3 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_span/src/def_id.rs:202:12
    |
202 |     pub fn new(crate_name: &str, is_exe: bool, mut metadata: Vec<String>) -> StableCrateId {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0432.
Some errors have detailed explanations: E0061, E0432.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_hir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_macros" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_resolve" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_builtin_macros" "-p" "rustc_ast_passes" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ty_utils" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_lint" "-p" "rustc_mir" "-p" "rustc_apfloat" "-p" "coverage_test_macros" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_session" "-p" "rustc_data_structures" "-p" "rustc_error_codes" "-p" "rustc_serialize" "-p" "rustc_typeck" "-p" "rustc_ast" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_mir_build" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:58
