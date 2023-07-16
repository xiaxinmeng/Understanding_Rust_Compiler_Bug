
time: 0.003; rss: 55MB	parse_crate
time: 0.001; rss: 55MB	attributes_injection
time: 0.001; rss: 55MB	recursion_limit
time: 0.001; rss: 57MB	plugin_loading
time: 0.000; rss: 57MB	plugin_registration
time: 0.003; rss: 57MB	pre_AST_expansion_lint_checks
time: 0.003; rss: 59MB	crate_injection
time: 0.008; rss: 124MB	pre_AST_expansion_lint_checks
time: 0.288; rss: 134MB	expand_crate
time: 0.000; rss: 134MB	check_unused_macros
time: 0.288; rss: 134MB	macro_expand_crate
time: 0.000; rss: 134MB	maybe_building_test_harness
time: 0.005; rss: 134MB	AST_validation
time: 0.000; rss: 134MB	maybe_create_a_macro_crate
time: 0.003; rss: 157MB	complete_gated_feature_checking
time: 0.450; rss: 157MB	configure_and_expand
time: 0.000; rss: 159MB	prepare_outputs
time: 0.060; rss: 178MB	hir_lowering
time: 0.014; rss: 178MB	early_lint_checks
time: 0.011; rss: 179MB	setup_global_ctxt
time: 0.000; rss: 179MB	dep_graph_tcx_init
time: 0.021; rss: 179MB	create_global_ctxt
time: 0.000; rss: 184MB	looking_for_entry_point
time: 0.000; rss: 184MB	looking_for_plugin_registrar
time: 0.001; rss: 184MB	looking_for_derive_registrar
time: 0.039; rss: 187MB	misc_checking_1
time: 0.039; rss: 189MB	type_collecting
time: 0.000; rss: 189MB	impl_wf_inference
time: 0.000; rss: 189MB	unsafety_checking
time: 0.000; rss: 189MB	orphan_checking
time: 0.000; rss: 189MB	coherence_checking
time: 0.104; rss: 213MB	wf_checking
time: 0.027; rss: 215MB	item_types_checking
time: 35.773; rss: 274MB	item_bodies_checking
time: 35.944; rss: 274MB	type_check_crate
time: 0.050; rss: 275MB	match_checking
time: 190.171; rss: 5505MB	liveness_and_intrinsic_checking
time: 190.261; rss: 5503MB	misc_checking_2
time: 3.083; rss: 591MB	MIR_borrow_checking
time: 0.000; rss: 591MB	MIR_effect_checking
time: 0.019; rss: 591MB	layout_testing
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

time: 0.037; rss: 594MB	death_checking
time: 0.006; rss: 594MB	unused_lib_feature_checking
time: 0.036; rss: 596MB	crate_lints
time: 0.066; rss: 597MB	module_lints
time: 0.102; rss: 597MB	lint_checking
time: 0.045; rss: 598MB	privacy_checking_modules
time: 0.220; rss: 598MB	misc_checking_3
time: 0.020; rss: 600MB	monomorphization_collector_root_collections
time: 0.372; rss: 641MB	monomorphization_collector_graph_walk
time: 0.010; rss: 642MB	partition_and_assert_distinct_symbols
time: 0.004; rss: 659MB	write_allocator_module
time: 0.000; rss: 659MB	find_cgu_reuse
time: 0.055; rss: 668MB	codegen_to_LLVM_IR
time: 0.001; rss: 668MB	assert_dep_graph
time: 0.000; rss: 668MB	serialize_dep_graph
time: 0.560; rss: 668MB	codegen_crate
time: 0.013; rss: 635MB	LLVM_module_optimize_function_passes(a.disqncfg-cgu.2)
time: 0.013; rss: 635MB	LLVM_module_optimize_function_passes(a.disqncfg-cgu.0)
time: 0.014; rss: 635MB	LLVM_module_optimize_function_passes(a.disqncfg-cgu.1)
time: 0.018; rss: 639MB	LLVM_module_optimize_module_passes(a.disqncfg-cgu.2)
time: 0.020; rss: 639MB	LLVM_module_optimize_module_passes(a.disqncfg-cgu.1)
time: 0.023; rss: 640MB	LLVM_module_optimize_module_passes(a.disqncfg-cgu.0)
time: 0.010; rss: 647MB	LLVM_lto_optimize(a.disqncfg-cgu.2)
time: 0.013; rss: 647MB	LLVM_lto_optimize(a.disqncfg-cgu.0)
time: 0.014; rss: 647MB	LLVM_lto_optimize(a.disqncfg-cgu.1)
time: 0.128; rss: 652MB	LLVM_passes(crate)
time: 0.129; rss: 649MB	free_global_ctxt
time: 0.000; rss: 649MB	join_worker_thread
time: 0.000; rss: 649MB	finish_ongoing_codegen
time: 0.000; rss: 649MB	llvm_dump_timing_file
time: 0.000; rss: 649MB	serialize_work_products
time: 0.000; rss: 649MB	link_binary_check_files_are_writeable
time: 0.476; rss: 650MB	run_linker
time: 0.000; rss: 650MB	link_binary_remove_temps
time: 0.484; rss: 650MB	link_binary
time: 0.484; rss: 650MB	link_crate
time: 0.485; rss: 650MB	link
warning: 5 warnings emitted

time: 231.413; rss: 650MB		total
    Finished release [optimized] target(s) in 3m 52s
