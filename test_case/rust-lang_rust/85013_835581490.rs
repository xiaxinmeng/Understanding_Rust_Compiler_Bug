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
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0599]: the method `last` exists for struct `bit_set::BitSet<usize>`, but its trait bounds were not satisfied
  --> compiler/rustc_index/src/bit_set/tests.rs:47:27
   |
47 |         assert_eq!(bitset.last(), Some(element));
   |                           ^^^^ method cannot be called on `bit_set::BitSet<usize>` due to unsatisfied trait bounds
  ::: compiler/rustc_index/src/bit_set.rs:31:1
   |
31 | pub struct BitSet<T> {
   | --------------------
   | --------------------
   | |
   | method `last` not found for this
   | doesn't satisfy `bit_set::BitSet<usize>: Iterator`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `bit_set::BitSet<usize>: Iterator`
           which is required by `&mut bit_set::BitSet<usize>: Iterator`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `last`, perhaps you need to implement it:
           candidate #1: `Iterator`

error[E0599]: the method `last` exists for struct `bit_set::BitSet<usize>`, but its trait bounds were not satisfied
  --> compiler/rustc_index/src/bit_set/tests.rs:50:23
   |
50 |     assert_eq!(bitset.last(), None);
   |                       ^^^^ method cannot be called on `bit_set::BitSet<usize>` due to unsatisfied trait bounds
  ::: compiler/rustc_index/src/bit_set.rs:31:1
   |
31 | pub struct BitSet<T> {
   | --------------------
   | --------------------
   | |
   | method `last` not found for this
   | doesn't satisfy `bit_set::BitSet<usize>: Iterator`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `bit_set::BitSet<usize>: Iterator`
           which is required by `&mut bit_set::BitSet<usize>: Iterator`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `last`, perhaps you need to implement it:
           candidate #1: `Iterator`
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_index`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_lexer" "-p" "rustc_fs_util" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_apfloat" "-p" "rustc_ast" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_index" "-p" "rustc_hir" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_query_impl" "-p" "rustc_trait_selection" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_lint" "-p" "rustc_typeck" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_mir_build" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "coverage_test_macros" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:44
