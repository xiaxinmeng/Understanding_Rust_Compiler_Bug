
~/src/issue66617 (master*) [11:26:27] 0
xtt$ rustc +8aa9d2014f4e5258f83b907e8431c59a33acdae7 -vV
rustc 1.43.0-nightly (8aa9d2014 2020-02-21)
binary: rustc
commit-hash: 8aa9d2014f4e5258f83b907e8431c59a33acdae7
commit-date: 2020-02-21
host: x86_64-apple-darwin
release: 1.43.0-nightly
LLVM version: 9.0
~/src/issue66617 (master*) [11:30:07] 1
xtt$ rustc +8aa9d2014f4e5258f83b907e8431c59a33acdae7 -C opt-level=3 -Ztime-passes ./src/main.rs
time: 0.006     parse_crate
time: 0.000     attributes_injection
time: 0.000     recursion_limit
time: 0.000     plugin_loading
time: 0.000     plugin_registration
time: 0.000     pre_AST_expansion_lint_checks
time: 0.000     crate_injection
time: 0.014     expand_crate
time: 0.000     check_unused_macros
time: 0.014     macro_expand_crate
time: 0.000     maybe_building_test_harness
time: 0.001     AST_validation
time: 0.000     maybe_create_a_macro_crate
time: 0.000     complete_gated_feature_checking
time: 0.017     configure_and_expand
time: 0.000     prepare_outputs
time: 0.002     hir_lowering
time: 0.000     early_lint_checks
time: 0.001     validate_HIR_map
time: 0.000     setup_global_ctxt
time: 0.000     dep_graph_tcx_init
time: 0.003     create_global_ctxt
time: 0.000     looking_for_entry_point
time: 0.000     looking_for_plugin_registrar
time: 0.000     looking_for_derive_registrar
time: 0.001     misc_checking_1
time: 0.006     type_collecting
time: 0.000     impl_wf_inference
time: 0.000     unsafety_checking
time: 0.000     orphan_checking
time: 0.002     coherence_checking
time: 0.007     wf_checking
time: 0.021     item_types_checking
time: 0.012     item_bodies_checking
time: 0.047     type_check_crate
time: 0.000     match_checking
time: 0.001     liveness_and_intrinsic_checking
time: 0.001     misc_checking_2
time: 0.005     MIR_borrow_checking
time: 0.000     dumping_chalk_like_clauses
time: 0.000     MIR_effect_checking
time: 0.000     layout_testing
time: 0.001     death_checking
time: 0.000     unused_lib_feature_checking
time: 0.005     crate_lints
time: 0.002     module_lints
time: 0.007     lint_checking
time: 0.002     privacy_checking_modules
time: 0.011     misc_checking_3
time: 0.000     monomorphization_collector_root_collections
time: 0.031     monomorphization_collector_graph_walk
time: 0.005     partition_and_assert_distinct_symbols
time: 0.000     write_allocator_module
time: 0.000     find_cgu_reuse
time: 0.031     LLVM_module_optimize_function_passes(main.7rcbfp3g-cgu.4)
time: 0.112     codegen_to_LLVM_IR
time: 0.000     assert_dep_graph
time: 0.000     serialize_dep_graph
time: 0.150     codegen_crate
time: 0.002     LLVM_module_optimize_function_passes(main.7rcbfp3g-cgu.1)
time: 0.001     LLVM_module_optimize_function_passes(main.7rcbfp3g-cgu.0)
time: 0.001     LLVM_module_optimize_function_passes(main.7rcbfp3g-cgu.3)
time: 0.001     LLVM_module_optimize_module_passes(main.7rcbfp3g-cgu.3)
time: 0.006     free_global_ctxt
time: 0.004     LLVM_module_optimize_module_passes(main.7rcbfp3g-cgu.0)
time: 0.011     LLVM_module_optimize_module_passes(main.7rcbfp3g-cgu.1)
time: 0.035     LLVM_module_optimize_function_passes(main.7rcbfp3g-cgu.2)
time: 10.943    LLVM_module_optimize_module_passes(main.7rcbfp3g-cgu.2)
