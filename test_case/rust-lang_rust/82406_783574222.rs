
$ cargo rustc --bin boolsatr -- -Z time-passes
   Compiling boolsatr v0.1.0 (/home/david/projects/boolsatr)
time:   0.006; rss:   50MB ->   52MB (   +1MB)	parse_crate
time:   0.001; rss:   52MB ->   52MB (   +0MB)	attributes_injection
time:   0.004; rss:   52MB ->   53MB (   +2MB)	incr_comp_prepare_session_directory
time:   0.001; rss:   53MB ->   53MB (   +0MB)	incr_comp_garbage_collect_session_directories
time:   0.001; rss:   53MB ->   53MB (   +0MB)	recursion_limit
time:   0.000; rss:   53MB ->   53MB (   +0MB)	plugin_loading
time:   0.000; rss:   53MB ->   53MB (   +0MB)	plugin_registration
time:   0.002; rss:   55MB ->   55MB (   +0MB)	crate_injection
time:   0.228; rss:   55MB ->   80MB (  +25MB)	expand_crate
time:   0.000; rss:   80MB ->   80MB (   +0MB)	check_unused_macros
time:   0.229; rss:   55MB ->   80MB (  +25MB)	macro_expand_crate
time:   0.000; rss:   80MB ->   80MB (   +0MB)	maybe_building_test_harness
time:   0.001; rss:   80MB ->   80MB (   +0MB)	AST_validation
time:   0.000; rss:   80MB ->   80MB (   +0MB)	maybe_create_a_macro_crate
time:   0.001; rss:   80MB ->   80MB (   +0MB)	finalize_imports
time:   0.001; rss:   80MB ->   81MB (   +0MB)	finalize_macro_resolutions
time:   0.023; rss:   81MB ->   84MB (   +3MB)	late_resolve_crate
time:   0.001; rss:   84MB ->   84MB (   +0MB)	resolve_check_unused
time:   0.000; rss:   84MB ->   84MB (   +0MB)	resolve_report_errors
time:   0.000; rss:   84MB ->   84MB (   +0MB)	resolve_postprocess
time:   0.026; rss:   80MB ->   84MB (   +3MB)	resolve_crate
time:   0.001; rss:   84MB ->   84MB (   +0MB)	complete_gated_feature_checking
time:   0.276; rss:   53MB ->   84MB (  +30MB)	configure_and_expand
time:   0.001; rss:   84MB ->   84MB (   +0MB)	prepare_outputs
time:   0.002; rss:   84MB ->   84MB (   +0MB)	blocked_on_dep_graph_loading
time:   0.009; rss:   84MB ->   87MB (   +3MB)	hir_lowering
time:   0.001; rss:   87MB ->   87MB (   +0MB)	early_lint_checks
time:   0.005; rss:   87MB ->   89MB (   +2MB)	setup_global_ctxt
time:   0.012; rss:   87MB ->   89MB (   +2MB)	create_global_ctxt
time:   0.003; rss:   92MB ->   92MB (   +0MB)	looking_for_entry_point
time:   0.001; rss:   92MB ->   92MB (   +0MB)	looking_for_plugin_registrar
time:   0.000; rss:   92MB ->   92MB (   +0MB)	looking_for_derive_registrar
time:   0.038; rss:   92MB ->   99MB (   +7MB)	misc_checking_1
time:   0.020; rss:   99MB ->  102MB (   +3MB)	type_collecting
time:   0.001; rss:  102MB ->  102MB (   +0MB)	impl_wf_inference
time:   0.000; rss:  139MB ->  139MB (   +0MB)	unsafety_checking
time:   0.000; rss:  139MB ->  139MB (   +0MB)	orphan_checking
time:   0.166; rss:  102MB ->  139MB (  +37MB)	coherence_checking
time:   0.043; rss:  139MB ->  144MB (   +5MB)	wf_checking
time:   0.003; rss:  144MB ->  144MB (   +0MB)	item_types_checking
time:   0.177; rss:  144MB ->  155MB (  +12MB)	item_bodies_checking
time:   0.409; rss:   99MB ->  155MB (  +57MB)	type_check_crate
time:   0.004; rss:  155MB ->  156MB (   +1MB)	match_checking
time:   0.004; rss:  156MB ->  158MB (   +2MB)	liveness_and_intrinsic_checking
time:   0.008; rss:  155MB ->  158MB (   +3MB)	misc_checking_2
time:   0.197; rss:  158MB ->  176MB (  +18MB)	MIR_borrow_checking
time:   0.000; rss:  176MB ->  176MB (   +0MB)	MIR_effect_checking
time:   0.000; rss:  176MB ->  176MB (   +0MB)	layout_testing
warning: associated function is never used: `dump_stats`
   --> src/solver/mod.rs:101:12
    |
101 |     pub fn dump_stats(&mut self, _graph_file: &mut File) -> Result<(), io::Error> {
    |            ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

time:   0.003; rss:  177MB ->  177MB (   +1MB)	death_checking
time:   0.001; rss:  177MB ->  177MB (   +0MB)	unused_lib_feature_checking
time:   0.003; rss:  177MB ->  179MB (   +2MB)	crate_lints
time:   0.002; rss:  179MB ->  179MB (   +0MB)	module_lints
time:   0.005; rss:  177MB ->  179MB (   +2MB)	lint_checking
time:   0.008; rss:  179MB ->  179MB (   +0MB)	privacy_checking_modules
time:   0.020; rss:  176MB ->  179MB (   +3MB)	misc_checking_3
time:   0.003; rss:  179MB ->  180MB (   +1MB)	monomorphization_collector_root_collections
time:  93.116; rss:  180MB ->  254MB (  +74MB)	monomorphization_collector_graph_walk
time:  45.622; rss:  254MB -> 7394MB (+7140MB)	partition_and_assert_distinct_symbols
time:   0.001; rss: 7395MB -> 7395MB (   +0MB)	write_allocator_module
time:   0.000; rss: 7395MB -> 7395MB (   +0MB)	find_cgu_reuse
time:   0.000; rss: 5826MB -> 5826MB (   +0MB)	LLVM_module_optimize_function_passes(2yqwcofw48oip3q)
time:   0.000; rss: 5826MB -> 5826MB (   +0MB)	LLVM_module_optimize_module_passes(2yqwcofw48oip3q)
time:   0.002; rss: 5826MB -> 5827MB (   +1MB)	LLVM_module_optimize_function_passes(5054vo9ldvu6shng)
time:   0.037; rss: 5827MB -> 5832MB (   +5MB)	LLVM_module_optimize_module_passes(5054vo9ldvu6shng)
time:   0.000; rss: 5859MB -> 5859MB (   +0MB)	LLVM_module_optimize_function_passes(483dpiq9qkl9z8jb)
time:   0.000; rss: 5859MB -> 5859MB (   +0MB)	LLVM_module_optimize_module_passes(483dpiq9qkl9z8jb)
time:   0.001; rss: 5859MB -> 5860MB (   +1MB)	LLVM_module_optimize_function_passes(2o3q8cccyyjhbuub)
time:   0.028; rss: 5860MB -> 5864MB (   +4MB)	LLVM_module_optimize_module_passes(2o3q8cccyyjhbuub)
time:   0.000; rss: 5905MB -> 5905MB (   +0MB)	LLVM_module_optimize_function_passes(51uxob6iog635dyd)
time:   0.000; rss: 5905MB -> 5905MB (   +0MB)	LLVM_module_optimize_module_passes(51uxob6iog635dyd)
time:   0.002; rss: 5905MB -> 5906MB (   +1MB)	LLVM_module_optimize_function_passes(4rj8oul8m6jyjmj1)
time:   0.044; rss: 5906MB -> 5914MB (   +8MB)	LLVM_module_optimize_module_passes(4rj8oul8m6jyjmj1)
time:   0.000; rss: 5921MB -> 5921MB (   +0MB)	LLVM_module_optimize_function_passes(42r33rw64tassjdf)
time:   0.000; rss: 5921MB -> 5921MB (   +0MB)	LLVM_module_optimize_function_passes(8q5lsqmu640v7oj)
time:   0.000; rss: 5921MB -> 5921MB (   +0MB)	LLVM_module_optimize_module_passes(8q5lsqmu640v7oj)
time:   0.009; rss: 5921MB -> 5922MB (   +2MB)	LLVM_module_optimize_module_passes(42r33rw64tassjdf)
time:   0.000; rss: 11669MB -> 11669MB (   +0MB)	LLVM_module_optimize_function_passes(2wu69c6uhy5ywqpg)
time:   0.000; rss: 11669MB -> 11669MB (   +0MB)	LLVM_module_optimize_module_passes(2wu69c6uhy5ywqpg)
time:   0.001; rss: 11669MB -> 11669MB (   +0MB)	LLVM_module_optimize_function_passes(z221iw9si33ns95)
time:   0.235; rss: 11669MB -> 11695MB (  +26MB)	LLVM_module_optimize_module_passes(z221iw9si33ns95)
time:   0.000; rss: 14330MB -> 14331MB (   +1MB)	LLVM_module_optimize_function_passes(9wflittwaemudxk)
time:   0.000; rss: 14331MB -> 14331MB (   +0MB)	LLVM_module_optimize_function_passes(389lxyptdtznlxxi)
time:   0.000; rss: 14331MB -> 14331MB (   +0MB)	LLVM_module_optimize_module_passes(389lxyptdtznlxxi)
time:   0.009; rss: 14331MB -> 14359MB (  +28MB)	LLVM_module_optimize_module_passes(9wflittwaemudxk)
^C  Building [========================>  ] 14/15: boolsatr(bin)                                                                                                                                                                           
