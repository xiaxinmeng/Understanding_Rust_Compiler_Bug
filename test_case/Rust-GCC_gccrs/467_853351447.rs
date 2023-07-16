
gccrs/gcc/testsuite/rust/compile/torture/arithmetic_expressions1.rs:3:1: internal compiler error: 'verify_gimple' failed
0x1a0d6e5 verify_gimple_in_seq(gimple*)
	../../gcc/tree-cfg.c:5157
0x16c9585 gimplify_body(tree_node*, bool)
	../../gcc/gimplify.c:15401
0x16c9857 gimplify_function_tree(tree_node*)
	../../gcc/gimplify.c:15472
0x14a1739 cgraph_node::analyze()
	../../gcc/cgraphunit.c:670
0x14a4635 analyze_functions
	../../gcc/cgraphunit.c:1236
0x14a5901 symbol_table::finalize_compilation_unit()
	../../gcc/cgraphunit.c:2514
Please submit a full bug report,
with preprocessed source if appropriate.
Please include the complete backtrace with any bug report.
See <https://gcc.gnu.org/bugs/> for instructions.
compiler exited with status 1
FAIL: rust/compile/torture/arithmetic_expressions1.rs   -O0  (internal compiler error)
