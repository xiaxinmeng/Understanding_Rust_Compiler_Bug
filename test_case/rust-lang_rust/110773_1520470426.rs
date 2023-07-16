plain
 finished in 0.476 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 246 tests
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFF  88/246
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFFFFFFFFFFiiFFFFiFiFFFFFFFFF 176/246
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiiFFFFFiFFFFFFFFFFFFFFFFFFFFFFFFFFFF...
failures:

---- [mir-opt] tests/mir-opt/asm_unwind_panic_abort.rs stdout ----
---- [mir-opt] tests/mir-opt/asm_unwind_panic_abort.rs stdout ----
thread '[mir-opt] tests/mir-opt/asm_unwind_panic_abort.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort/asm_unwind_panic_abort.main.AbortUnwindingCalls.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/array_index_is_temporary.rs stdout ----
---- [mir-opt] tests/mir-opt/array_index_is_temporary.rs stdout ----
thread '[mir-opt] tests/mir-opt/array_index_is_temporary.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/array_index_is_temporary/array_index_is_temporary.main.SimplifyCfg-elaborate-drops.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/array_index_is_temporary`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/address_of.rs stdout ----
---- [mir-opt] tests/mir-opt/address_of.rs stdout ----
thread '[mir-opt] tests/mir-opt/address_of.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/address_of/address_of.address_of_reborrow.SimplifyCfg-initial.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/address_of`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/bool_compare.rs stdout ----
---- [mir-opt] tests/mir-opt/bool_compare.rs stdout ----
thread '[mir-opt] tests/mir-opt/bool_compare.rs' panicked at 'the mir dump file for bool_compare.opt1.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/bool_compare.rs)', src/tools/compiletest/src/runtest.rs:3600:17

---- [mir-opt] tests/mir-opt/basic_assignment.rs stdout ----
---- [mir-opt] tests/mir-opt/basic_assignment.rs stdout ----
thread '[mir-opt] tests/mir-opt/basic_assignment.rs' panicked at 'the mir dump file for basic_assignment.main.ElaborateDrops.before.mir does not exist (requested in /checkout/tests/mir-opt/basic_assignment.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/box_expr.rs stdout ----
---- [mir-opt] tests/mir-opt/box_expr.rs stdout ----
thread '[mir-opt] tests/mir-opt/box_expr.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr/box_expr.main.ElaborateDrops.before.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/arrays.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/arrays.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/arrays.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/arrays/arrays.arrays.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/arrays`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/arbitrary_let.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/arbitrary_let.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/arbitrary_let.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/arbitrary_let/arbitrary_let.arbitrary_let.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/arbitrary_let`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/async_await.rs stdout ----
---- [mir-opt] tests/mir-opt/building/async_await.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/async_await.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/async_await/async_await.a-{closure#0}.generator_resume.0.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/async_await`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/composite_return.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/composite_return.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/composite_return.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/composite_return/composite_return.tuple.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/composite_return`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/aggregate_exprs.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/aggregate_exprs.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/aggregate_exprs.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/aggregate_exprs/aggregate_exprs.tuple.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/aggregate_exprs`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/operators.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/operators.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/operators.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/operators/operators.f.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/operators`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/consts.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/consts.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/consts.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/consts/consts.consts.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/consts`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/as_cast.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/as_cast.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/as_cast.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/as_cast/as_cast.int_to_int.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/as_cast`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/issue_101867.rs stdout ----
---- [mir-opt] tests/mir-opt/building/issue_101867.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/issue_101867.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/issue_101867/issue_101867.main.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/issue_101867`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/simple_assign.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/simple_assign.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/simple_assign.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/simple_assign/simple_assign.simple.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/simple_assign`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/byte_slice.rs stdout ----
---- [mir-opt] tests/mir-opt/byte_slice.rs stdout ----
thread '[mir-opt] tests/mir-opt/byte_slice.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/byte_slice/byte_slice.main.SimplifyCfg-elaborate-drops.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/byte_slice`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/combine_array_len.rs stdout ----
---- [mir-opt] tests/mir-opt/combine_array_len.rs stdout ----
thread '[mir-opt] tests/mir-opt/combine_array_len.rs' panicked at 'the mir dump file for combine_array_len.norm2.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/combine_array_len.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/casts.rs stdout ----
---- [mir-opt] tests/mir-opt/casts.rs stdout ----
thread '[mir-opt] tests/mir-opt/casts.rs' panicked at 'the mir dump file for casts.redundant.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/casts.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/building/storage_live_dead_in_statics.rs stdout ----
---- [mir-opt] tests/mir-opt/building/storage_live_dead_in_statics.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/storage_live_dead_in_statics.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/storage_live_dead_in_statics/storage_live_dead_in_statics.XXX.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/storage_live_dead_in_statics`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/combine_clone_of_primitives.rs stdout ----
---- [mir-opt] tests/mir-opt/combine_clone_of_primitives.rs stdout ----
thread '[mir-opt] tests/mir-opt/combine_clone_of_primitives.rs' panicked at 'the mir dump file for combine_clone_of_primitives.{impl#0}-clone.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/combine_clone_of_primitives.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/building/receiver_ptr_mutability.rs stdout ----
---- [mir-opt] tests/mir-opt/building/receiver_ptr_mutability.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/receiver_ptr_mutability.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/receiver_ptr_mutability/receiver_ptr_mutability.main.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/receiver_ptr_mutability`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/enums.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/enums.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/enums.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/enums/enums.switch_bool.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/enums`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/projections.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/projections.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/projections.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/projections/projections.unions.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/projections`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/references.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/references.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/references.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/references/references.mut_ref.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/references`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/custom/terminators.rs stdout ----
---- [mir-opt] tests/mir-opt/building/custom/terminators.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/custom/terminators.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/terminators/terminators.direct_call.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/custom/terminators`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/combine_transmutes.rs stdout ----
---- [mir-opt] tests/mir-opt/combine_transmutes.rs stdout ----
thread '[mir-opt] tests/mir-opt/combine_transmutes.rs' panicked at 'the mir dump file for combine_transmutes.identity_transmutes.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/combine_transmutes.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/building/shifts.rs stdout ----
---- [mir-opt] tests/mir-opt/building/shifts.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/shifts.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/shifts/shifts.shift_signed.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/shifts`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/enum_cast.rs stdout ----
---- [mir-opt] tests/mir-opt/building/enum_cast.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/enum_cast.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/enum_cast/enum_cast.foo.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/enum_cast`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_debuginfo.rs' panicked at 'the mir dump file for const_debuginfo.main.ConstDebugInfo.before.mir does not exist (requested in /checkout/tests/mir-opt/const_debuginfo.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_allocation.rs stdout ----
---- [mir-opt] tests/mir-opt/const_allocation.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_allocation.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation/const_allocation.main.ConstProp.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/building/simple_match.rs stdout ----
---- [mir-opt] tests/mir-opt/building/simple_match.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/simple_match.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/simple_match/simple_match.match_bool.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/simple_match`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/const_goto.rs stdout ----
---- [mir-opt] tests/mir-opt/const_goto.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_goto.rs' panicked at 'the mir dump file for const_goto.issue_77355_opt.ConstGoto.before.mir does not exist (requested in /checkout/tests/mir-opt/const_goto.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_allocation2.rs stdout ----
---- [mir-opt] tests/mir-opt/const_allocation2.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_allocation2.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2/const_allocation2.main.ConstProp.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/const_goto_storage.rs stdout ----
---- [mir-opt] tests/mir-opt/const_goto_storage.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_goto_storage.rs' panicked at 'the mir dump file for const_goto_storage.match_nested_if.ConstGoto.before.mir does not exist (requested in /checkout/tests/mir-opt/const_goto_storage.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/building/issue_49232.rs stdout ----
---- [mir-opt] tests/mir-opt/building/issue_49232.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/issue_49232.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/issue_49232/issue_49232.main.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/issue_49232`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/const_prop/array_index.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/array_index.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/array_index.rs' panicked at 'the mir dump file for array_index.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/array_index.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_goto_const_eval_fail.rs stdout ----
---- [mir-opt] tests/mir-opt/const_goto_const_eval_fail.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_goto_const_eval_fail.rs' panicked at 'the mir dump file for const_goto_const_eval_fail.f.ConstGoto.before.mir does not exist (requested in /checkout/tests/mir-opt/const_goto_const_eval_fail.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/building/match_false_edges.rs stdout ----
---- [mir-opt] tests/mir-opt/building/match_false_edges.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/match_false_edges.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/match_false_edges/match_false_edges.full_tested_match.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/match_false_edges`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/const_prop/aggregate.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/aggregate.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/aggregate.rs' panicked at 'the mir dump file for aggregate.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/aggregate.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/bad_op_div_by_zero.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/bad_op_div_by_zero.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/bad_op_div_by_zero.rs' panicked at 'the mir dump file for bad_op_div_by_zero.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/bad_op_div_by_zero.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/bad_op_mod_by_zero.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/bad_op_mod_by_zero.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/bad_op_mod_by_zero.rs' panicked at 'the mir dump file for bad_op_mod_by_zero.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/bad_op_mod_by_zero.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/building/uniform_array_move_out.rs stdout ----
---- [mir-opt] tests/mir-opt/building/uniform_array_move_out.rs stdout ----
thread '[mir-opt] tests/mir-opt/building/uniform_array_move_out.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/uniform_array_move_out/uniform_array_move_out.move_out_from_end.built.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/building/uniform_array_move_out`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/const_allocation3.rs stdout ----
---- [mir-opt] tests/mir-opt/const_allocation3.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_allocation3.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation3/const_allocation3.main.ConstProp.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation3`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/const_prop/boolean_identities.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/boolean_identities.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/boolean_identities.rs' panicked at 'the mir dump file for boolean_identities.test.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/boolean_identities.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/boxes.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/boxes.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/boxes.rs' panicked at 'the mir dump file for boxes.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/boxes.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_promotion_extern_static.rs stdout ----
---- [mir-opt] tests/mir-opt/const_promotion_extern_static.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_promotion_extern_static.rs' panicked at 'the mir dump file for const_promotion_extern_static.BAR.PromoteTemps.before.mir does not exist (requested in /checkout/tests/mir-opt/const_promotion_extern_static.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/cast.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/cast.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/cast.rs' panicked at 'the mir dump file for cast.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/cast.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'the mir dump file for const_prop_fails_gracefully.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/const_prop_fails_gracefully.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'the mir dump file for bad_op_unsafe_oob_for_slices.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs' panicked at 'the mir dump file for inherit_overflow.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/inherit_overflow.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/discriminant.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/discriminant.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/discriminant.rs' panicked at 'the mir dump file for discriminant.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/discriminant.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/issue_66971.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/issue_66971.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/issue_66971.rs' panicked at 'the mir dump file for issue_66971.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/issue_66971.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/large_array_index.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/large_array_index.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/large_array_index.rs' panicked at 'the mir dump file for large_array_index.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/large_array_index.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/issue_67019.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/issue_67019.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/issue_67019.rs' panicked at 'the mir dump file for issue_67019.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/issue_67019.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/indirect.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/indirect.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/indirect.rs' panicked at 'the mir dump file for indirect.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/indirect.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/mult_by_zero.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/mult_by_zero.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/mult_by_zero.rs' panicked at 'the mir dump file for mult_by_zero.test.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/mult_by_zero.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate.rs' panicked at 'the mir dump file for mutable_variable_aggregate.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/mutable_variable_aggregate.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs' panicked at 'the mir dump file for mutable_variable_aggregate_partial_read.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs' panicked at 'the mir dump file for control_flow_simplification.hello.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/control_flow_simplification.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/invalid_constant.rs' panicked at 'the mir dump file for invalid_constant.main.RemoveZsts.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/invalid_constant.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_no_prop.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_no_prop.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/mutable_variable_no_prop.rs' panicked at 'the mir dump file for mutable_variable_no_prop.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/mutable_variable_no_prop.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/mutable_variable.rs' panicked at 'the mir dump file for mutable_variable.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/mutable_variable.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/checked_add.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/checked_add.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/checked_add.rs' panicked at 'the mir dump file for checked_add.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/checked_add.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate_mut_ref.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate_mut_ref.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/mutable_variable_aggregate_mut_ref.rs' panicked at 'the mir dump file for mutable_variable_aggregate_mut_ref.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/mutable_variable_aggregate_mut_ref.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_unprop_assign.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/mutable_variable_unprop_assign.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/mutable_variable_unprop_assign.rs' panicked at 'the mir dump file for mutable_variable_unprop_assign.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/mutable_variable_unprop_assign.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/offset_of.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/offset_of.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/offset_of.rs' panicked at 'the mir dump file for offset_of.concrete.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/offset_of.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/read_immutable_static.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/read_immutable_static.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/read_immutable_static.rs' panicked at 'the mir dump file for read_immutable_static.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/read_immutable_static.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/ref_deref_project.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/ref_deref_project.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/ref_deref_project.rs' panicked at 'the mir dump file for ref_deref_project.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/ref_deref_project.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/ref_deref.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/ref_deref.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/ref_deref.rs' panicked at 'the mir dump file for ref_deref.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/ref_deref.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/reify_fn_ptr.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/reify_fn_ptr.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/reify_fn_ptr.rs' panicked at 'the mir dump file for reify_fn_ptr.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/reify_fn_ptr.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/repeat.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/repeat.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/repeat.rs' panicked at 'the mir dump file for repeat.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/repeat.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/scalar_literal_propagation.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/scalar_literal_propagation.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/scalar_literal_propagation.rs' panicked at 'the mir dump file for scalar_literal_propagation.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/scalar_literal_propagation.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/slice_len.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/slice_len.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/slice_len.rs' panicked at 'the mir dump file for slice_len.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/slice_len.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/switch_int.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/switch_int.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/switch_int.rs' panicked at 'the mir dump file for switch_int.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/switch_int.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop_miscompile.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop_miscompile.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop_miscompile.rs' panicked at 'the mir dump file for const_prop_miscompile.foo.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop_miscompile.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/tuple_literal_propagation.rs' panicked at 'the mir dump file for tuple_literal_propagation.main.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/tuple_literal_propagation.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/return_place.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/return_place.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/return_place.rs' panicked at 'the mir dump file for return_place.add.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/return_place.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/transmute.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/transmute.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/transmute.rs' panicked at 'the mir dump file for transmute.less_as_i8.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/transmute.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/borrowed_local.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/borrowed_local.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/borrowed_local.rs' panicked at 'the mir dump file for borrowed_local.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/borrowed_local.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/branch.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/branch.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/branch.rs' panicked at 'the mir dump file for branch.foo.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/branch.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/copy_propagation_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/copy_propagation_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/copy_propagation_arg.rs' panicked at 'the mir dump file for copy_propagation_arg.foo.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/copy_propagation_arg.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/const_prop/optimizes_into_variable.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/optimizes_into_variable.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'the mir dump file for optimizes_into_variable.main.ScalarReplacementOfAggregates.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/optimizes_into_variable.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/custom_move_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/custom_move_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/custom_move_arg.rs' panicked at 'the mir dump file for custom_move_arg.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/custom_move_arg.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/cycle.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/cycle.rs' panicked at 'the mir dump file for cycle.main.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/cycle.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/issue_107511.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/issue_107511.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/issue_107511.rs' panicked at 'the mir dump file for issue_107511.main.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/issue_107511.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/move_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/move_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/move_arg.rs' panicked at 'the mir dump file for move_arg.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/move_arg.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/move_projection.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/move_projection.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/move_projection.rs' panicked at 'the mir dump file for move_projection.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/move_projection.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/non_dominate.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/non_dominate.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/non_dominate.rs' panicked at 'the mir dump file for non_dominate.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/non_dominate.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/reborrow.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/reborrow.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/reborrow.rs' panicked at 'the mir dump file for reborrow.remut.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/reborrow.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/checked.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/checked.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/checked.rs' panicked at 'the mir dump file for checked.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/checked.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_79191.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_79191.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/dead_stores_79191.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_79191/dead_stores_79191.f.CopyProp.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_79191`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/dataflow-const-prop/cast.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/cast.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/cast.rs' panicked at 'the mir dump file for cast.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/cast.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/mutate_through_pointer.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/mutate_through_pointer.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/mutate_through_pointer.rs' panicked at 'the mir dump file for mutate_through_pointer.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/mutate_through_pointer.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_better.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_better.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/dead_stores_better.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_better/dead_stores_better.f.CopyProp.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_better`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/dataflow-const-prop/if.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/if.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/if.rs' panicked at 'the mir dump file for if.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/if.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/enum.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/enum.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/enum.rs' panicked at 'the mir dump file for enum.simple.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/enum.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs' panicked at 'the mir dump file for inherit_overflow.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/inherit_overflow.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/issue_81605.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/issue_81605.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/issue_81605.rs' panicked at 'the mir dump file for issue_81605.f.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/issue_81605.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/repr_transparent.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/repr_transparent.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/repr_transparent.rs' panicked at 'the mir dump file for repr_transparent.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/repr_transparent.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/self_assign_add.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/self_assign_add.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/self_assign_add.rs' panicked at 'the mir dump file for self_assign_add.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/self_assign_add.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/struct.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/struct.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/struct.rs' panicked at 'the mir dump file for struct.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/struct.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/terminator.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/terminator.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/terminator.rs' panicked at 'the mir dump file for terminator.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/terminator.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/tuple.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/tuple.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/tuple.rs' panicked at 'the mir dump file for tuple.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/tuple.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dead-store-elimination/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/dead-store-elimination/cycle.rs stdout ----
thread '[mir-opt] tests/mir-opt/dead-store-elimination/cycle.rs' panicked at 'the mir dump file for cycle.cycle.DeadStoreElimination.before.mir does not exist (requested in /checkout/tests/mir-opt/dead-store-elimination/cycle.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dead-store-elimination/provenance_soundness.rs stdout ----
---- [mir-opt] tests/mir-opt/dead-store-elimination/provenance_soundness.rs stdout ----
thread '[mir-opt] tests/mir-opt/dead-store-elimination/provenance_soundness.rs' panicked at 'the mir dump file for provenance_soundness.pointer_to_int.DeadStoreElimination.before.mir does not exist (requested in /checkout/tests/mir-opt/dead-store-elimination/provenance_soundness.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/deduplicate_blocks.rs stdout ----
---- [mir-opt] tests/mir-opt/deduplicate_blocks.rs stdout ----
thread '[mir-opt] tests/mir-opt/deduplicate_blocks.rs' panicked at 'the mir dump file for deduplicate_blocks.is_line_doc_comment_2.DeduplicateBlocks.before.mir does not exist (requested in /checkout/tests/mir-opt/deduplicate_blocks.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/derefer_complex_case.rs stdout ----
---- [mir-opt] tests/mir-opt/derefer_complex_case.rs stdout ----
thread '[mir-opt] tests/mir-opt/derefer_complex_case.rs' panicked at 'the mir dump file for derefer_complex_case.main.Derefer.before.mir does not exist (requested in /checkout/tests/mir-opt/derefer_complex_case.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/derefer_inline_test.rs stdout ----
---- [mir-opt] tests/mir-opt/derefer_inline_test.rs stdout ----
thread '[mir-opt] tests/mir-opt/derefer_inline_test.rs' panicked at 'the mir dump file for derefer_inline_test.main.Derefer.before.mir does not exist (requested in /checkout/tests/mir-opt/derefer_inline_test.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/derefer_terminator_test.rs stdout ----
---- [mir-opt] tests/mir-opt/derefer_terminator_test.rs stdout ----
thread '[mir-opt] tests/mir-opt/derefer_terminator_test.rs' panicked at 'the mir dump file for derefer_terminator_test.main.Derefer.before.mir does not exist (requested in /checkout/tests/mir-opt/derefer_terminator_test.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/derefer_test.rs stdout ----
---- [mir-opt] tests/mir-opt/derefer_test.rs stdout ----
thread '[mir-opt] tests/mir-opt/derefer_test.rs' panicked at 'the mir dump file for derefer_test.main.Derefer.before.mir does not exist (requested in /checkout/tests/mir-opt/derefer_test.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/derefer_test_multiple.rs stdout ----
---- [mir-opt] tests/mir-opt/derefer_test_multiple.rs stdout ----
thread '[mir-opt] tests/mir-opt/derefer_test_multiple.rs' panicked at 'the mir dump file for derefer_test_multiple.main.Derefer.before.mir does not exist (requested in /checkout/tests/mir-opt/derefer_test_multiple.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/deref-patterns/string.rs stdout ----
---- [mir-opt] tests/mir-opt/deref-patterns/string.rs stdout ----
thread '[mir-opt] tests/mir-opt/deref-patterns/string.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deref-patterns/string/string.foo.PreCodegen.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deref-patterns/string`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/dest-prop/copy_propagation_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/copy_propagation_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/copy_propagation_arg.rs' panicked at 'the mir dump file for copy_propagation_arg.foo.DestinationPropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/dest-prop/copy_propagation_arg.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dest-prop/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/cycle.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/cycle.rs' panicked at 'the mir dump file for cycle.main.DestinationPropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/dest-prop/cycle.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dest-prop/simple.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/simple.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/simple.rs' panicked at 'the mir dump file for simple.nrvo.DestinationPropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/dest-prop/simple.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dest-prop/union.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/union.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/union.rs' panicked at 'the mir dump file for union.main.DestinationPropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/dest-prop/union.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dest-prop/unreachable.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/unreachable.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/unreachable.rs' panicked at 'the mir dump file for unreachable.f.DestinationPropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/dest-prop/unreachable.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/early_otherwise_branch.rs stdout ----
---- [mir-opt] tests/mir-opt/early_otherwise_branch.rs stdout ----
thread '[mir-opt] tests/mir-opt/early_otherwise_branch.rs' panicked at 'the mir dump file for early_otherwise_branch.opt1.EarlyOtherwiseBranch.before.mir does not exist (requested in /checkout/tests/mir-opt/early_otherwise_branch.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/ref_without_sb.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/ref_without_sb.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/ref_without_sb.rs' panicked at 'the mir dump file for ref_without_sb.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/ref_without_sb.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dest-prop/dead_stores_better.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/dead_stores_better.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/dead_stores_better.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/dest-prop/dead_stores_better/dead_stores_better.f.DestinationPropagation.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/dest-prop/dead_stores_better`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/early_otherwise_branch_68867.rs stdout ----
---- [mir-opt] tests/mir-opt/early_otherwise_branch_68867.rs stdout ----
thread '[mir-opt] tests/mir-opt/early_otherwise_branch_68867.rs' panicked at 'the mir dump file for early_otherwise_branch_68867.try_sum.EarlyOtherwiseBranch.before.mir does not exist (requested in /checkout/tests/mir-opt/early_otherwise_branch_68867.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dest-prop/dead_stores_79191.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/dead_stores_79191.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/dead_stores_79191.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/dest-prop/dead_stores_79191/dead_stores_79191.f.DestinationPropagation.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/dest-prop/dead_stores_79191`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/dest-prop/branch.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/branch.rs stdout ----
thread '[mir-opt] tests/mir-opt/dest-prop/branch.rs' panicked at 'the mir dump file for branch.foo.DestinationPropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/dest-prop/branch.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dont_yeet_assert.rs stdout ----
---- [mir-opt] tests/mir-opt/dont_yeet_assert.rs stdout ----
thread '[mir-opt] tests/mir-opt/dont_yeet_assert.rs' panicked at 'the mir dump file for dont_yeet_assert.generic.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/dont_yeet_assert.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/early_otherwise_branch_3_element_tuple.rs stdout ----
---- [mir-opt] tests/mir-opt/early_otherwise_branch_3_element_tuple.rs stdout ----
thread '[mir-opt] tests/mir-opt/early_otherwise_branch_3_element_tuple.rs' panicked at 'the mir dump file for early_otherwise_branch_3_element_tuple.opt1.EarlyOtherwiseBranch.before.mir does not exist (requested in /checkout/tests/mir-opt/early_otherwise_branch_3_element_tuple.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/sibling_ptr.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/sibling_ptr.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/sibling_ptr.rs' panicked at 'the mir dump file for sibling_ptr.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/sibling_ptr.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/self_assign.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/self_assign.rs stdout ----
thread '[mir-opt] tests/mir-opt/dataflow-const-prop/self_assign.rs' panicked at 'the mir dump file for self_assign.main.DataflowConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/dataflow-const-prop/self_assign.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/early_otherwise_branch_noopt.rs stdout ----
---- [mir-opt] tests/mir-opt/early_otherwise_branch_noopt.rs stdout ----
thread '[mir-opt] tests/mir-opt/early_otherwise_branch_noopt.rs' panicked at 'the mir dump file for early_otherwise_branch_noopt.noopt1.EarlyOtherwiseBranch.before.mir does not exist (requested in /checkout/tests/mir-opt/early_otherwise_branch_noopt.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/enum_opt.rs stdout ----
---- [mir-opt] tests/mir-opt/enum_opt.rs stdout ----
thread '[mir-opt] tests/mir-opt/enum_opt.rs' panicked at 'the mir dump file for enum_opt.unin.EnumSizeOpt.before.mir does not exist (requested in /checkout/tests/mir-opt/enum_opt.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/equal_true.rs stdout ----
---- [mir-opt] tests/mir-opt/equal_true.rs stdout ----
thread '[mir-opt] tests/mir-opt/equal_true.rs' panicked at 'the mir dump file for equal_true.opt.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/equal_true.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/funky_arms.rs stdout ----
---- [mir-opt] tests/mir-opt/funky_arms.rs stdout ----
thread '[mir-opt] tests/mir-opt/funky_arms.rs' panicked at 'the mir dump file for funky_arms.float_to_exponential_common.ConstProp.before.mir does not exist (requested in /checkout/tests/mir-opt/funky_arms.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/early_otherwise_branch_soundness.rs stdout ----
---- [mir-opt] tests/mir-opt/early_otherwise_branch_soundness.rs stdout ----
thread '[mir-opt] tests/mir-opt/early_otherwise_branch_soundness.rs' panicked at 'the mir dump file for early_otherwise_branch_soundness.no_downcast.EarlyOtherwiseBranch.before.mir does not exist (requested in /checkout/tests/mir-opt/early_otherwise_branch_soundness.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/asm_unwind.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/asm_unwind.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/asm_unwind.rs' panicked at 'the mir dump file for asm_unwind.main.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/asm_unwind.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/generator_drop_cleanup.rs stdout ----
---- [mir-opt] tests/mir-opt/generator_drop_cleanup.rs stdout ----
thread '[mir-opt] tests/mir-opt/generator_drop_cleanup.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_drop_cleanup/generator_drop_cleanup.main-{closure#0}.generator_drop.0.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_drop_cleanup`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/caller_with_trivial_bound.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/caller_with_trivial_bound.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/caller_with_trivial_bound.rs' panicked at 'the mir dump file for caller_with_trivial_bound.foo.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/caller_with_trivial_bound.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/if_condition_int.rs stdout ----
---- [mir-opt] tests/mir-opt/if_condition_int.rs stdout ----
thread '[mir-opt] tests/mir-opt/if_condition_int.rs' panicked at 'the mir dump file for if_condition_int.opt_u32.SimplifyComparisonIntegral.before.mir does not exist (requested in /checkout/tests/mir-opt/if_condition_int.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/generator_storage_dead_unwind.rs stdout ----
---- [mir-opt] tests/mir-opt/generator_storage_dead_unwind.rs stdout ----
thread '[mir-opt] tests/mir-opt/generator_storage_dead_unwind.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_storage_dead_unwind/generator_storage_dead_unwind.main-{closure#0}.StateTransform.before.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_storage_dead_unwind`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/dyn_trait.rs' panicked at 'the mir dump file for dyn_trait.mk_cycle.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/dyn_trait.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/cycle.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/cycle.rs' panicked at 'the mir dump file for cycle.f.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/cycle.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/fn_ptr_shim.rs stdout ----
---- [mir-opt] tests/mir-opt/fn_ptr_shim.rs stdout ----
thread '[mir-opt] tests/mir-opt/fn_ptr_shim.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/fn_ptr_shim/core.ops-function-Fn-call.AddMovesForPackedDrops.before.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/fn_ptr_shim`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/generator_tiny.rs stdout ----
---- [mir-opt] tests/mir-opt/generator_tiny.rs stdout ----
thread '[mir-opt] tests/mir-opt/generator_tiny.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny/generator_tiny.main-{closure#0}.generator_resume.0.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator_tiny`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/exponential_or.rs stdout ----
---- [mir-opt] tests/mir-opt/exponential_or.rs stdout ----
thread '[mir-opt] tests/mir-opt/exponential_or.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/exponential_or/exponential_or.match_tuple.SimplifyCfg-initial.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/exponential_or`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/inline_closure_borrows_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_closure_borrows_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_closure_borrows_arg.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_closure_borrows_arg/inline_closure_borrows_arg.foo.Inline.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_closure_borrows_arg`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/graphviz.rs stdout ----
---- [mir-opt] tests/mir-opt/graphviz.rs stdout ----
thread '[mir-opt] tests/mir-opt/graphviz.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/graphviz/graphviz.main.built.after.dot` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/graphviz`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/inline_closure.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_closure.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_closure.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_closure/inline_closure.foo.Inline.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_closure`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/exponential_runtime.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/exponential_runtime.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/exponential_runtime.rs' panicked at 'the mir dump file for exponential_runtime.main.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/exponential_runtime.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_shims.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_shims.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_shims.rs' panicked at 'the mir dump file for inline_shims.clone.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_shims.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_compatibility.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_compatibility.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_compatibility.rs' panicked at 'the mir dump file for inline_compatibility.inlined_target_feature.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_compatibility.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_closure_captures.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_closure_captures.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_closure_captures.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_closure_captures/inline_closure_captures.foo.Inline.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_closure_captures`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/inline_retag.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_retag.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_retag.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_retag/inline_retag.bar.Inline.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_retag`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_cycle.rs' panicked at 'the mir dump file for inline_cycle.one.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_cycle.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_cycle_generic.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_cycle_generic.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_cycle_generic.rs' panicked at 'the mir dump file for inline_cycle_generic.main.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_cycle_generic.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_any_operand.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_any_operand.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_any_operand.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_any_operand/inline_any_operand.bar.Inline.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_any_operand`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_generator.rs' panicked at 'the mir dump file for inline_generator.main.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_generator.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_specialization.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_specialization.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_specialization.rs' panicked at 'the mir dump file for inline_specialization.main.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_specialization.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/issue_106141.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/issue_106141.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/issue_106141.rs' panicked at 'the mir dump file for issue_106141.outer.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/issue_106141.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_trait_method.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_trait_method.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_trait_method.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_trait_method/inline_trait_method.test.Inline.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_trait_method`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/inline_options.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_options.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_options.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_options/inline_options.main.Inline.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline_options`', src/tools/compiletest/src/runtest.rs:3565:21
---- [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_instruction_set.rs' panicked at 'the mir dump file for inline_instruction_set.t32.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_instruction_set.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_diverging.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_diverging.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/inline_diverging.rs' panicked at 'the mir dump file for inline_diverging.f.Inline.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/inline_diverging.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/issue_78442.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/issue_78442.rs stdout ----
thread '[mir-opt] tests/mir-opt/inline/issue_78442.rs' panicked at 'the mir dump file for issue_78442.bar.RevealAll.before.mir does not exist (requested in /checkout/tests/mir-opt/inline/issue_78442.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/instcombine_duplicate_switch_targets.rs stdout ----
---- [mir-opt] tests/mir-opt/instcombine_duplicate_switch_targets.rs stdout ----
thread '[mir-opt] tests/mir-opt/instcombine_duplicate_switch_targets.rs' panicked at 'the mir dump file for instcombine_duplicate_switch_targets.assert_zero.InstCombine.before.mir does not exist (requested in /checkout/tests/mir-opt/instcombine_duplicate_switch_targets.rs)', src/tools/compiletest/src/runtest.rs:3600:17
---- [mir-opt] tests/mir-opt/inline/inline_trait_method_2.rs stdout ----
