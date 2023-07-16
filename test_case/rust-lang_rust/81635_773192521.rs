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
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0433]: failed to resolve: use of undeclared type `Fingerprint`
   --> compiler/rustc_hir/src/definitions.rs:514:43
    |
514 |         let d0 = CrateDisambiguator::from(Fingerprint::new(12, 34));
    |
help: consider importing one of these items
    |
502 |     use crate::def_id::Fingerprint;
502 |     use crate::def_id::Fingerprint;
    |
502 |     use rustc_data_structures::fingerprint::Fingerprint;
    |

error[E0433]: failed to resolve: use of undeclared type `Fingerprint`
   --> compiler/rustc_hir/src/definitions.rs:515:43
    |
515 |         let d1 = CrateDisambiguator::from(Fingerprint::new(56, 78));
    |
help: consider importing one of these items
    |
502 |     use crate::def_id::Fingerprint;
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_incremental" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_symbol_mangling" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:12
