
   Compiling a v0.1.0 (/Users/yupferris/dev/projects/a)
time: 0.001	parse_crate
time: 0.000	attributes_injection
time: 0.000	recursion_limit
time: 0.000	plugin_loading
time: 0.000	plugin_registration
time: 0.000	pre_AST_expansion_lint_checks
time: 0.000	crate_injection
time: 0.013	pre_AST_expansion_lint_checks
time: 0.219	expand_crate
time: 0.000	check_unused_macros
time: 0.219	macro_expand_crate
time: 0.000	maybe_building_test_harness
time: 0.003	AST_validation
time: 0.000	maybe_create_a_macro_crate
time: 0.005	complete_gated_feature_checking
time: 0.357	configure_and_expand
time: 0.001	prepare_outputs
time: 0.071	hir_lowering
time: 0.023	early_lint_checks
time: 0.001	setup_global_ctxt
time: 0.000	dep_graph_tcx_init
time: 0.001	create_global_ctxt
time: 0.000	looking_for_entry_point
time: 0.000	looking_for_plugin_registrar
time: 0.000	looking_for_derive_registrar
time: 0.012	misc_checking_1
time: 0.009	type_collecting
time: 0.000	impl_wf_inference
time: 0.000	unsafety_checking
time: 0.000	orphan_checking
time: 0.000	coherence_checking
time: 0.047	wf_checking
time: 0.003	item_types_checking
time: 70.603	item_bodies_checking
time: 70.662	type_check_crate
time: 0.147	match_checking
time: 14.560	liveness_and_intrinsic_checking
time: 14.707	misc_checking_2
time: 0.849	MIR_borrow_checking
time: 0.000	MIR_effect_checking
time: 0.000	layout_testing
warning: struct is never constructed: `Top`
 --> src/modules.rs:1:12
  |
1 | pub struct Top{
  |            ^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: associated function is never used: `new`
    --> src/modules.rs:1358:12
     |
1358 |     pub fn new() -> Top {
     |            ^^^

warning: associated function is never used: `reset`
    --> src/modules.rs:3006:12
     |
3006 |     pub fn reset(&mut self) {
     |            ^^^^^

warning: associated function is never used: `posedge_clk`
    --> src/modules.rs:3128:12
     |
3128 |     pub fn posedge_clk(&mut self) {
     |            ^^^^^^^^^^^

warning: associated function is never used: `prop`
    --> src/modules.rs:3707:12
     |
3707 |     pub fn prop(&mut self) {
     |            ^^^^

time: 0.005	death_checking
time: 0.002	unused_lib_feature_checking
time: 0.006	crate_lints
time: 0.039	module_lints
time: 0.045	lint_checking
time: 0.039	privacy_checking_modules
time: 0.096	misc_checking_3
time: 0.001	monomorphization_collector_root_collections
time: 0.017	monomorphization_collector_graph_walk
time: 0.001	partition_and_assert_distinct_symbols
time: 0.000	write_allocator_module
time: 0.000	find_cgu_reuse
time: 0.001	LLVM_module_optimize_function_passes(a.6tst2r86-cgu.1)
time: 0.004	codegen_to_LLVM_IR
time: 0.000	assert_dep_graph
time: 0.000	serialize_dep_graph
time: 0.024	codegen_crate
time: 0.001	LLVM_module_optimize_function_passes(a.6tst2r86-cgu.2)
time: 0.001	LLVM_module_optimize_function_passes(a.6tst2r86-cgu.0)
time: 0.003	LLVM_module_optimize_module_passes(a.6tst2r86-cgu.1)
time: 0.001	LLVM_module_optimize_module_passes(a.6tst2r86-cgu.0)
time: 0.004	LLVM_module_optimize_module_passes(a.6tst2r86-cgu.2)
time: 0.003	LLVM_lto_optimize(a.6tst2r86-cgu.0)
time: 0.004	LLVM_lto_optimize(a.6tst2r86-cgu.1)
time: 0.005	LLVM_lto_optimize(a.6tst2r86-cgu.2)
time: 0.020	LLVM_passes(crate)
time: 0.078	free_global_ctxt
time: 0.000	join_worker_thread
time: 0.000	finish_ongoing_codegen
time: 0.000	llvm_dump_timing_file
time: 0.000	serialize_work_products
time: 0.000	link_binary_check_files_are_writeable
time: 0.386	run_linker
time: 0.000	link_binary_remove_temps
time: 0.387	link_binary
time: 0.387	link_crate
time: 0.388	link
warning: 5 warnings emitted

time: 87.377		total
    Finished release [optimized] target(s) in 1m 27s
