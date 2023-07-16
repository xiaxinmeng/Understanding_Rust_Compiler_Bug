plain
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
   Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 53.65s
Testing ["alloc", "core", "panic_abort", "panic_unwind", "proc_macro", "std", "test", "unwind"] stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

 finished in 282.408 seconds
Testing ["rustc-main", "rustc_apfloat", "rustc_arena", "rustc_ast", "rustc_ast_lowering", "rustc_ast_passes", "rustc_ast_pretty", "rustc_attr", "rustc_borrowck", "rustc_builtin_macros", "rustc_codegen_llvm", "rustc_codegen_ssa", "rustc_const_eval", "rustc_data_structures", "rustc_driver", "rustc_error_codes", "rustc_error_messages", "rustc_errors", "rustc_expand", "rustc_feature", "rustc_fs_util", "rustc_graphviz", "rustc_hir", "rustc_hir_pretty", "rustc_incremental", "rustc_index", "rustc_infer", "rustc_interface", "rustc_lexer", "rustc_lint", "rustc_lint_defs", "rustc_llvm", "rustc_log", "rustc_macros", "rustc_metadata", "rustc_middle", "rustc_mir_build", "rustc_mir_dataflow", "rustc_mir_transform", "coverage_test_macros", "rustc_monomorphize", "rustc_parse", "rustc_parse_format", "rustc_passes", "rustc_plugin_impl", "rustc_privacy", "rustc_query_impl", "rustc_query_system", "rustc_resolve", "rustc_save_analysis", "rustc_serialize", "rustc_session", "rustc_smir", "rustc_span", "rustc_symbol_mangling", "rustc_target", "rustc_trait_selection", "rustc_traits", "rustc_ty_utils", "rustc_type_ir", "rustc_typeck"] stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
   Compiling diff v0.1.12
   Compiling ansi_term v0.12.1
   Compiling pretty_assertions v0.7.2
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
144 |         first(cache.all::<compile::Std>()),
    |                     ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`

error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
161 |             first(cache.all::<compile::Std>()),
    |                         ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`
error[E0277]: the trait bound `compile::Rustc: std::cmp::Ord` is not satisfied
   --> builder/tests.rs:172:32
    |
    |
172 |         assert_eq!(first(cache.all::<compile::Rustc>()), &[rustc!(A => A, stage = 0)],);
    |                                ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Rustc`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`

error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
181 |         assert_eq!(first(cache.all::<compile::Std>()), &[std!(A => A, stage = 0)]);
    |                                ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`
error[E0277]: the trait bound `compile::Rustc: std::cmp::Ord` is not satisfied
   --> builder/tests.rs:189:23
    |
    |
189 |         assert!(cache.all::<compile::Rustc>().is_empty());
    |                       ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Rustc`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`

error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
206 |             first(cache.all::<compile::Std>()),
    |                         ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`
error[E0277]: the trait bound `compile::Rustc: std::cmp::Ord` is not satisfied
   --> builder/tests.rs:230:25
    |
    |
230 |             first(cache.all::<compile::Rustc>()),
    |                         ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Rustc`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`

error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
352 |             first(cache.all::<compile::Std>()),
    |                         ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`
error[E0277]: the trait bound `compile::Rustc: std::cmp::Ord` is not satisfied
   --> builder/tests.rs:378:25
    |
    |
378 |             first(cache.all::<compile::Rustc>()),
    |                         ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Rustc`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`

error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
464 |             first(cache.all::<compile::Std>()),
    |                         ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`

error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
494 |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`
error[E0277]: the trait bound `compile::Rustc: std::cmp::Ord` is not satisfied
   --> builder/tests.rs:506:33
    |
    |
506 |             first(builder.cache.all::<compile::Rustc>()),
    |                                 ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Rustc`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`

error[E0277]: the trait bound `compile::Std: std::cmp::Ord` is not satisfied
    |
    |
527 |             first(builder.cache.all::<compile::Std>()),
    |                                 ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Std`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`
error[E0277]: the trait bound `compile::Rustc: std::cmp::Ord` is not satisfied
   --> builder/tests.rs:539:33
    |
    |
539 |             first(builder.cache.all::<compile::Rustc>()),
    |                                 ^^^ the trait `std::cmp::Ord` is not implemented for `compile::Rustc`
note: required by a bound in `cache::Cache::all`
   --> cache.rs:296:19
    |
    |
296 |     pub fn all<S: Ord + Clone + Step>(&mut self) -> Vec<(S, S::Output)> {
    |                   ^^^ required by this bound in `cache::Cache::all`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to 14 previous errors
Build completed unsuccessfully in 0:29:45
