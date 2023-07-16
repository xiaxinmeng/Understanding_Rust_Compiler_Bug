text
running 20 tests
test borrow_tracker::tree_borrows::perms::propagation_optimization_checks::foreign_read_is_noop_after_write ... ok
test borrow_tracker::tree_borrows::perms::propagation_optimization_checks::access_transitions_progress_increasing ... ok
test borrow_tracker::tree_borrows::perms::propagation_optimization_checks::all_transitions_idempotent ... ok
test borrow_tracker::tree_borrows::unimap::tests::consistency_small ... ok
test borrow_tracker::tree_borrows::unimap::tests::consistency_large ... ok
test borrow_tracker::tree_borrows::unimap::tests::extend_to_length ... ok
test concurrency::range_object_map::tests::boundaries ... ok
test concurrency::range_object_map::tests::empty_map ... ok
test concurrency::range_object_map::tests::perfectly_overlapping ... ok
test concurrency::range_object_map::tests::straddling ... ok
test concurrency::vector_clock::tests::test_equal ... ok
test concurrency::vector_clock::tests::test_partial_order ... ok
test eval::tests::windows_argv0_no_escape ... ok
test intptrcast::tests::test_align_addr ... ok
test range_map::tests::basic_insert ... ok
test range_map::tests::gaps ... ok
test eval::tests::windows_argv0_panic_on_quote - should panic ... ok
test range_map::tests::out_of_range_iter_mut - should panic ... ok
test range_map::tests::out_of_range_iter - should panic ... ok
test concurrency::range_object_map::tests::no_overlapping_inserts - should panic ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/compiletest.rs (build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-43d30f56be80d5b6)
## Running ui tests in tests/pass against miri for x86_64-unknown-linux-gnu
   Compiler flags: ["--error-format=json", "--edition", "2018", "-Astable-features", "-Aunused", "-Zui-testing", "--target", "x86_64-unknown-linux-gnu"]
tests/pass/align.rs ... ok
tests/pass/arrays.rs ... ok
tests/pass/assume_bug.rs (stack) ... ok
tests/pass/assume_bug.rs (tree) ... ok
tests/pass/async-fn.rs ... ok
tests/pass/associated-const.rs (stack) ... ok
tests/pass/associated-const.rs (tree) ... ok
tests/pass/align_offset_symbolic.rs ... ok
tests/pass/atomic-compare-exchange-weak-never-fail.rs ... ok
tests/pass/available-parallelism-miri-num-cpus.rs ... ok
tests/pass/bad_substs.rs ... ok
tests/pass/available-parallelism.rs (stack) ... ok
tests/pass/available-parallelism.rs (tree) ... ok
tests/pass/binops.rs ... ok
tests/pass/atomic.rs (stack) ... ok
tests/pass/atomic.rs (tree) ... ok
tests/pass/bools.rs ... ok
tests/pass/adjacent-allocs.rs (stack) ... ok
tests/pass/adjacent-allocs.rs (tree) ... ok
tests/pass/binary-heap.rs ... ok
tests/pass/c_enums.rs ... ok
tests/pass/calls.rs ... ok
tests/pass/cast_fn_ptr.rs ... ok
tests/pass/box-custom-alloc.rs (stack) ... ok
tests/pass/box-custom-alloc.rs (tree) ... ok
tests/pass/cast-rfc0401-vtable-kinds.rs (stack) ... ok
tests/pass/cast-rfc0401-vtable-kinds.rs (tree) ... ok
tests/pass/cast_fn_ptr_unsafe.rs ... ok
tests/pass/catch.rs ... ok
tests/pass/cfg_miri.rs ... ok
tests/pass/box.rs (stack) ... ok
tests/pass/box.rs (tree) ... ok
tests/pass/char.rs ... ok
tests/pass/closure-drop.rs ... ok
tests/pass/closure-field-ty.rs ... ok
tests/pass/coercions.rs ... ok
tests/pass/closures.rs ... ok
tests/pass/coerce_non_capture_closure_to_fn_ptr.rs ... ok
tests/pass/const-vec-of-fns.rs ... ok
tests/pass/constants.rs ... ok
tests/pass/deriving-associated-types.rs ... ok
tests/pass/drop_empty_slice.rs ... ok
tests/pass/drop_on_fat_ptr_array_elements.rs ... ok
tests/pass/drop_on_array_elements.rs ... ok
tests/pass/disable-alignment-check.rs (stack) ... ok
tests/pass/disable-alignment-check.rs (tree) ... ok
tests/pass/drop_through_owned_slice.rs ... ok
tests/pass/drop_on_zst_array_elements.rs ... ok
tests/pass/drop_through_trait_object.rs ... ok
tests/pass/drop_through_trait_object_rc.rs ... ok
tests/pass/dst-field-align.rs ... ok
tests/pass/dst-irrefutable-bind.rs ... ok
tests/pass/dst-raw.rs ... ok
tests/pass/dst-struct-sole.rs ... ok
tests/pass/dyn-traits.rs ... ok
tests/pass/dyn-star.rs ... ok
tests/pass/dst-struct.rs ... ok
tests/pass/enum_discriminant_ptr_value.rs ... ok
tests/pass/enum-nullable-const-null-with-fields.rs ... ok
tests/pass/enums.rs ... ok
tests/pass/dyn-upcast.rs ... ok
tests/pass/extern_crate_std_in_main.rs ... ok
tests/pass/dyn-arbitrary-self.rs (stack) ... ok
tests/pass/dyn-arbitrary-self.rs (tree) ... ok
tests/pass/fat_ptr.rs ... ok
tests/pass/extern_types.rs (stack) ... ok
tests/pass/extern_types.rs (tree) ... ok
tests/pass/float_fast_math.rs ... ok
tests/pass/from_utf8.rs ... ok
tests/pass/format.rs ... ok
tests/pass/function_pointers.rs ... ok
tests/pass/getpid.rs ... ok
tests/pass/future-self-referential.rs (stack) ... ok
tests/pass/future-self-referential.rs (tree) ... ok
tests/pass/global_allocator.rs ... ok
tests/pass/generator.rs (stack) ... ok
tests/pass/generator.rs (tree) ... ok
tests/pass/heap.rs ... ok
tests/pass/hello.rs ... ok
tests/pass/hide_stdout.rs ... ok
tests/pass/integer-ops.rs ... ok
tests/pass/float.rs ... ok
tests/pass/hashmap.rs (stack) ... ok
tests/pass/hashmap.rs (tree) ... ok
tests/pass/heap_allocator.rs ... ok
tests/pass/intrinsics-integer.rs ... ok
tests/pass/intrinsics-math.rs ... ok
tests/pass/ints.rs ... ok
tests/pass/intrinsics.rs ... ok
tests/pass/intrinsics-x86.rs ... ok
tests/pass/intptrcast.rs (stack) ... ok
tests/pass/intptrcast.rs (tree) ... ok
tests/pass/iter.rs ... ok
tests/pass/last-use-in-cap-clause.rs ... ok
tests/pass/btreemap.rs (stack) ... ok
tests/pass/btreemap.rs (tree) ... ok
tests/pass/leak-in-static.rs ... ok
tests/pass/main_fn.rs ... ok
tests/pass/loop-break-value.rs ... ok
tests/pass/main_result.rs ... ok
tests/pass/loops.rs ... ok
tests/pass/match_slice.rs ... ok
tests/pass/miri-alloc.rs ... ok
tests/pass/memleak_ignored.rs (stack) ... ok
tests/pass/memleak_ignored.rs (tree) ... ok
tests/pass/many_shr_bor.rs (stack) ... ok
tests/pass/many_shr_bor.rs (tree) ... ok
tests/pass/memchr.rs ... ok
tests/pass/linked-list.rs (stack) ... ok
tests/pass/linked-list.rs (tree) ... ok
tests/pass/move-arg-2-unique.rs ... ok
tests/pass/move-arg-3-unique.rs ... ok
tests/pass/move-uninit-primval.rs ... ok
tests/pass/move-data-across-await-point.rs ... ok
tests/pass/mpsc.rs ... ok
tests/pass/no_std.rs ... ok
tests/pass/negative_discriminant.rs ... ok
tests/pass/multi_arg_closure.rs ... ok
tests/pass/observed_local_mut.rs ... ok
tests/pass/overflow_checks_off.rs ... ok
tests/pass/overloaded-calls-simple.rs ... ok
tests/pass/option_eq.rs ... ok
tests/pass/option_box_transmute_ptr.rs (stack) ... ok
tests/pass/option_box_transmute_ptr.rs (tree) ... ok
tests/pass/packed_struct.rs ... ok
tests/pass/partially-uninit.rs ... ok
tests/pass/products.rs ... ok
tests/pass/pointers.rs (stack) ... ok
tests/pass/pointers.rs (tree) ... ok
tests/pass/portable-simd.rs ... ok
tests/pass/ptr_int_casts.rs (stack) ... ok
tests/pass/ptr_int_casts.rs (tree) ... ok
tests/pass/provenance.rs (stack) ... ok
tests/pass/provenance.rs (tree) ... ok
tests/pass/ptr_int_from_exposed.rs (stack) ... ok
tests/pass/ptr_int_from_exposed.rs (tree) ... ok
tests/pass/ptr_raw.rs ... ok
tests/pass/ptr_offset.rs ... ok
tests/pass/ptr_int_transmute.rs (stack) ... ok
tests/pass/ptr_int_transmute.rs (tree) ... ok
tests/pass/recursive_static.rs ... ok
tests/pass/regions-lifetime-nonfree-late-bound.rs ... ok
tests/pass/reentrant-println.rs ... ok
tests/pass/rename_std.rs ... ok
tests/pass/rfc1623.rs ... ok
tests/pass/rust-lang-org.rs ... ok
tests/pass/sendable-class.rs ... ok
tests/pass/simd-intrinsic-generic-elements.rs ... ok
tests/pass/small_enum_size_bug.rs ... ok
tests/pass/rc.rs (stack) ... ok
tests/pass/rc.rs (tree) ... ok
tests/pass/send-is-not-static-par-for.rs (stack) ... ok
tests/pass/send-is-not-static-par-for.rs (tree) ... ok
tests/pass/start.rs ... ok
tests/pass/specialization.rs ... ok
tests/pass/static_memory_modification.rs ... ok
tests/pass/static_mut.rs ... ok
tests/pass/subslice_array.rs ... ok
tests/pass/strings.rs ... ok
tests/pass/sums.rs ... ok
tests/pass/tag-align-dyn-u64.rs ... ok
tests/pass/too-large-primval-write-problem.rs ... ok
tests/pass/track-caller-attribute.rs ... ok
tests/pass/trivial.rs ... ok
tests/pass/try-operator-custom.rs ... ok
tests/pass/slices.rs (stack) ... ok
tests/pass/slices.rs (tree) ... ok
tests/pass/transmute_ptr.rs (stack) ... ok
tests/pass/transmute_ptr.rs (tree) ... ok
tests/pass/tuple_like_enum_variant_constructor.rs ... ok
tests/pass/tuple_like_enum_variant_constructor_pointer_opt.rs ... ok
tests/pass/tuple_like_struct_constructor.rs ... ok
tests/pass/union-overwrite.rs ... ok
tests/pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs ... ok
tests/pass/union.rs ... ok
tests/pass/threadleak_ignored.rs (stack) ... ok
tests/pass/threadleak_ignored.rs (tree) ... ok
tests/pass/unops.rs ... ok
tests/pass/u128.rs ... ok
tests/pass/validation_lifetime_resolution.rs ... ok
tests/pass/vec-matching-fold.rs ... ok
tests/pass/volatile.rs ... ok
tests/pass/without-validation.rs ... ok
tests/pass/wtf8.rs ... ignored (in-test comment)
tests/pass/write-bytes.rs ... ok
tests/pass/zst.rs ... ok
tests/pass/unsized.rs (stack) ... ok
tests/pass/unsized.rs (tree) ... ok
tests/pass/zst_box.rs ... ok
tests/pass/zst_variant_drop.rs ... ok
tests/pass/vecdeque.rs (stack) ... ok
tests/pass/vecdeque.rs (tree) ... ok
tests/pass/vec.rs (stack) ... ok
tests/pass/vec.rs (tree) ... ok
tests/pass/backtrace/backtrace-api-v0.rs ... ok
tests/pass/backtrace/backtrace-api-v1.rs ... ok
tests/pass/concurrency/concurrent_caller_location.rs ... ok
tests/pass/concurrency/disable_data_race_detector.rs ... ok
tests/pass/concurrency/data_race.rs ... ok
tests/pass/concurrency/mutex_leak.rs ... ok
tests/pass/concurrency/issue1643.rs ... ok
tests/pass/concurrency/scope.rs ... ok
tests/pass/concurrency/spin_loop.rs ... ok
tests/pass/concurrency/simple.rs ... ok
tests/pass/backtrace/backtrace-global-alloc.rs ... ok
tests/pass/concurrency/spin_loops_nopreempt.rs ... ok
tests/pass/concurrency/sync_nopreempt.rs ... ok
tests/pass/concurrency/sync_singlethread.rs ... ok
tests/pass/backtrace/backtrace-std.rs ... ok
tests/pass/concurrency/thread_park_isolated.rs ... ok
tests/pass/concurrency/channels.rs (stack) ... ok
tests/pass/concurrency/channels.rs (tree) ... ok
tests/pass/concurrency/windows_condvar_shared.rs ... ignored (in-test comment)
tests/pass/concurrency/windows_detach_terminated.rs ... ignored (in-test comment)
tests/pass/concurrency/windows_init_once.rs ... ignored (in-test comment)
tests/pass/concurrency/windows_join_multiple.rs ... ignored (in-test comment)
tests/pass/concurrency/thread_locals.rs (stack) ... ok
tests/pass/concurrency/thread_locals.rs (tree) ... ok
tests/pass/function_calls/disable_abi_check.rs ... ok
tests/pass/concurrency/tls_lib_drop_single_thread.rs ... ok
tests/pass/function_calls/exported_symbol.rs ... ok
tests/pass/issues/issue-15063.rs ... ok
tests/pass/issues/issue-15080.rs ... ok
tests/pass/issues/issue-17877.rs ... ok
tests/pass/issues/issue-15523-big.rs ... ok
tests/pass/issues/issue-23261.rs ... ok
tests/pass/issues/issue-20575.rs ... ok
tests/pass/issues/issue-26709.rs ... ok
tests/pass/issues/issue-27901.rs ... ok
tests/pass/issues/issue-29746.rs ... ok
tests/pass/issues/issue-30530.rs ... ok
tests/pass/issues/issue-31267-additional.rs ... ok
tests/pass/issues/issue-33387.rs ... ok
tests/pass/issues/issue-34571.rs ... ok
tests/pass/issues/issue-35815.rs ... ok
tests/pass/issues/issue-36278-prefix-nesting.rs ... ok
tests/pass/issues/issue-53728.rs ... ok
tests/pass/issues/issue-3794.rs ... ok
tests/pass/issues/issue-5917.rs ... ok
tests/pass/issues/issue-73223.rs ... ok
tests/pass/issues/issue-94371.rs ... ok
tests/pass/issues/issue-91636.rs ... ok
tests/pass/issues/issue-miri-1075.rs ... ok
tests/pass/issues/issue-miri-184.rs ... ok
tests/pass/issues/issue-miri-133.rs ... ok
tests/pass/issues/issue-miri-1909.rs ... ok
tests/pass/issues/issue-miri-1925.rs ... ok
tests/pass/issues/issue-miri-2123.rs ... ok
tests/pass/issues/issue-miri-2068.rs ... ok
tests/pass/panic/std-panic-locations.rs ... ok
tests/pass/panic/concurrent-panic.rs ... ok
tests/pass/shims/exit.rs ... ok
tests/pass/0weak_memory_consistency.rs ... ok
tests/pass/shims/fs-with-isolation.rs ... ok
tests/pass/shims/io.rs ... ok
tests/pass/shims/ptr_mask.rs ... ok
tests/pass/concurrency/tls_lib_drop.rs (stack) ... ok
tests/pass/concurrency/tls_lib_drop.rs (tree) ... ok
tests/pass/panic/catch_panic.rs ... ok
tests/pass/shims/sleep_long.rs ... ok
tests/pass/stacked-borrows/2phase.rs ... ok
tests/pass/stacked-borrows/generators-self-referential.rs ... ok
tests/pass/shims/time.rs ... ok
tests/pass/stacked-borrows/int-to-ptr.rs ... ok
tests/pass/stacked-borrows/interior_mutability.rs ... ok
tests/pass/stacked-borrows/issue-miri-2389.rs ... ok
tests/pass/stacked-borrows/no_field_retagging.rs ... ok
tests/pass/stacked-borrows/non_scalar_field_retagging.rs ... ok
tests/pass/stacked-borrows/stack-printing.rs ... ok
tests/pass/concurrency/sync.rs (stack) ... ok
tests/pass/concurrency/sync.rs (tree) ... ok
tests/pass/stacked-borrows/stacked-borrows.rs ... ok
tests/pass/stacked-borrows/unknown-bottom-gc.rs ... ok
tests/pass/stacked-borrows/zst-field-retagging-terminates.rs ... ok
tests/pass/tree-borrows/2phase-interiormut.rs ... ok
tests/pass/tree-borrows/cell-alternate-writes.rs ... ok
tests/pass/shims/time-with-isolation.rs ... ok
tests/pass/tree-borrows/copy-nonoverlapping.rs ... ok
tests/pass/shims/fs.rs ... ok
tests/pass/tree-borrows/end-of-protector.rs ... ok
tests/pass/tree-borrows/formatting.rs ... ok
tests/pass/tree-borrows/read-only-from-mut.rs ... ok
tests/pass/tree-borrows/reborrow-is-read.rs ... ok
tests/pass/tree-borrows/reserved.rs ... ok
tests/pass/tree-borrows/transmute-unsafecell.rs ... ok
tests/pass/weak_memory/extra_cpp.rs ... ok
tests/pass/weak_memory/extra_cpp_unsafe.rs ... ok
tests/pass/shims/env/args.rs ... ok
tests/pass/shims/env/current_dir.rs ... ok
tests/pass/shims/env/current_dir_with_isolation.rs ... ok
tests/pass/shims/env/current_exe.rs ... ok
tests/pass/shims/env/home.rs ... ok
tests/pass/shims/env/var-forward.rs ... ok
tests/pass/shims/env/var-without-isolation.rs ... ok
tests/pass/weak_memory/weak.rs ... ok
tests/pass/shims/env/var.rs ... ok
tests/pass/shims/time-with-isolation2.rs ... ok

test result: ok. 292 tests passed, 5 ignored, 0 filtered out

## Running ui tests in tests/pass-dep against miri for x86_64-unknown-linux-gnu
   Compiler flags: ["--error-format=json", "--edition", "2018", "-Astable-features", "-Aunused", "-Zui-testing", "--target", "x86_64-unknown-linux-gnu"]
   Building test dependencies...
tests/pass-dep/num_cpus.rs ... ok
tests/pass-dep/getrandom_1.rs ... ok
tests/pass-dep/page_size.rs ... ok
tests/pass-dep/foreign-fn-linkname.rs ... ok
tests/pass-dep/calloc.rs ... ok
tests/pass-dep/malloc.rs ... ok
tests/pass-dep/page_size_override.rs ... ok
tests/pass-dep/regions-mock-trans.rs ... ok
tests/pass-dep/concurrency/libc_pthread_cond_isolated.rs ... ok
tests/pass-dep/shims/fcntl_f-fullfsync_apple.rs ... ignored (in-test comment)
tests/pass-dep/concurrency/tls_pthread_drop_order.rs ... ok
tests/pass-dep/shims/libc-fs-with-isolation.rs ... ok
tests/pass-dep/shims/env-cleanup-data-race.rs ... ok
tests/pass-dep/random.rs ... ok
tests/pass-dep/shims/libc-getrandom-without-isolation.rs ... ok
tests/pass-dep/shims/libc-getrandom.rs ... ok
tests/pass-dep/shims/posix_memalign.rs ... ok
tests/pass-dep/shims/libc-fs.rs ... ok
tests/pass-dep/shims/pthreads.rs ... ok
tests/pass-dep/shims/libc-misc.rs ... ok
tests/pass-dep/tokio/tokio_mvp.rs ... ok
tests/pass-dep/concurrency/linux-futex.rs ... ok
tests/pass-dep/concurrency/libc_pthread_cond.rs ... ok
tests/pass-dep/tokio/sleep.rs ... ok

test result: ok. 23 tests passed, 1 ignored, 0 filtered out

## Running ui tests in tests/panic against miri for x86_64-unknown-linux-gnu
   Compiler flags: ["--error-format=json", "--edition", "2018", "-Astable-features", "-Aunused", "-Zui-testing", "--target", "x86_64-unknown-linux-gnu"]
   Building test dependencies...
tests/panic/overflowing-lsh-neg.rs ... ok
tests/panic/overflowing-rsh-1.rs ... ok
tests/panic/overflowing-rsh-2.rs ... ok
tests/panic/panic2.rs ... ok
tests/panic/div-by-zero-2.rs ... ok
tests/panic/unsupported_foreign_function.rs ... ok
tests/panic/unsupported_syscall.rs ... ok
tests/panic/transmute_fat2.rs ... ok
tests/panic/panic3.rs ... ok
tests/panic/panic4.rs ... ok
tests/panic/function_calls/exported_symbol_good_unwind.rs ... ok
tests/panic/panic1.rs ... ok

test result: ok. 12 tests passed, 0 ignored, 0 filtered out

## Running ui tests in tests/fail against miri for x86_64-unknown-linux-gnu
   Compiler flags: ["--error-format=json", "--edition", "2018", "-Astable-features", "-Aunused", "-Zui-testing", "--target", "x86_64-unknown-linux-gnu"]
   Building test dependencies...
tests/fail/const-ub-checks.rs ... ok
tests/fail/breakpoint.rs ... ok
tests/fail/branchless-select-i128-pointer.rs ... ok
tests/fail/dyn-call-trait-mismatch.rs ... ok
tests/fail/box-cell-alias.rs ... ok
tests/fail/dyn-upcast-trait-mismatch.rs ... ok
tests/fail/erroneous_const2.rs ... ok
tests/fail/environ-gets-deallocated.rs ... ok
tests/fail/erroneous_const.rs ... ok
tests/fail/extern_static.rs ... ok
tests/fail/extern_static_in_const.rs ... ok
tests/fail/extern_static_wrong_size.rs ... ok
tests/fail/fast_math_both.rs ... ok
tests/fail/fast_math_first.rs ... ok
tests/fail/fast_math_second.rs ... ok
tests/fail/generator-pinned-moved.rs ... ok
tests/fail/invalid_char.rs ... ok
tests/fail/invalid_bool.rs ... ok
tests/fail/invalid_enum_tag.rs ... ok
tests/fail/invalid_int.rs ... ok
tests/fail/issue-miri-1112.rs ... ok
tests/fail/issue-miri-2432.rs ... ok
tests/fail/memleak_no_backtrace.rs ... ok
tests/fail/memleak.rs ... ok
tests/fail/modifying_constants.rs ... ok
tests/fail/no_main.rs ... ok
tests/fail/memleak_rc.rs ... ok
tests/fail/never_say_never.rs ... ok
tests/fail/never_transmute_humans.rs ... ok
tests/fail/never_transmute_void.rs ... ok
tests/fail/rc_as_ptr.rs ... ok
tests/fail/rustc-error.rs ... ok
tests/fail/reading_half_a_pointer.rs ... ok
tests/fail/static_memory_modification1.rs ... ok
tests/fail/static_memory_modification2.rs ... ok
tests/fail/static_memory_modification3.rs ... ok
tests/fail/type-too-large.rs ... ok
tests/fail/uninit_buffer.rs ... ok
tests/fail/transmute-pair-uninit.rs ... ok
tests/fail/uninit_byte_read.rs ... ok
tests/fail/uninit_buffer_with_provenance.rs ... ok
tests/fail/terminate-terminator.rs ... ok
tests/fail/unreachable.rs ... ok
tests/fail/unsupported_foreign_function.rs ... ok
tests/fail/unsized-local.rs ... ok
tests/fail/unsupported_incomplete_function.rs ... ok
tests/fail/zst1.rs ... ok
tests/fail/unwind-action-terminate.rs ... ok
tests/fail/zst2.rs ... ok
tests/fail/zst3.rs ... ok
tests/fail/alloc/deallocate-bad-alignment.rs ... ok
tests/fail/alloc/no_global_allocator.rs ... ok
tests/fail/alloc/deallocate-bad-size.rs ... ok
tests/fail/alloc/deallocate-twice.rs ... ok
tests/fail/alloc/global_system_mixup.rs ... ok
tests/fail/alloc/reallocate-bad-size.rs ... ok
tests/fail/alloc/reallocate-change-alloc.rs ... ok
tests/fail/alloc/stack_free.rs ... ok
tests/fail/alloc/reallocate-dangling.rs ... ok
tests/fail/concurrency/libc_pthread_create_too_few_args.rs ... ok
tests/fail/concurrency/libc_pthread_create_too_many_args.rs ... ok
tests/fail/concurrency/libc_pthread_join_detached.rs ... ok
tests/fail/concurrency/libc_pthread_create_main_terminate.rs ... ok
tests/fail/concurrency/libc_pthread_join_joined.rs ... ok
tests/fail/concurrency/read_only_atomic_load.rs ... ok
tests/fail/concurrency/read_only_atomic_cmpxchg.rs ... ok
tests/fail/concurrency/libc_pthread_join_main.rs ... ok
tests/fail/concurrency/windows_join_detached.rs ... ignored (in-test comment)
tests/fail/concurrency/windows_join_main.rs ... ignored (in-test comment)
tests/fail/concurrency/windows_join_self.rs ... ignored (in-test comment)
tests/fail/concurrency/libc_pthread_join_multiple.rs ... ok
tests/fail/concurrency/libc_pthread_join_self.rs ... ok
tests/fail/concurrency/thread_local_static_dealloc.rs ... ok
tests/fail/dangling_pointers/dangling_pointer_deref_underscore.rs ... ok
tests/fail/dangling_pointers/dangling_pointer_addr_of.rs ... ok
tests/fail/dangling_pointers/dangling_pointer_deref.rs ... ok
tests/fail/dangling_pointers/dangling_zst_deref.rs ... ok
tests/fail/dangling_pointers/deref-invalid-ptr.rs ... ok
tests/fail/dangling_pointers/dyn_size.rs ... ok
tests/fail/concurrency/unwind_top_of_stack.rs ... ok
tests/fail/dangling_pointers/deref-partially-dangling.rs ... ok
tests/fail/dangling_pointers/maybe_null_pointer_deref_zst.rs ... ok
tests/fail/dangling_pointers/maybe_null_pointer_write_zst.rs ... ok
tests/fail/dangling_pointers/null_pointer_deref.rs ... ok
tests/fail/dangling_pointers/null_pointer_deref_zst.rs ... ok
tests/fail/dangling_pointers/null_pointer_write_zst.rs ... ok
tests/fail/dangling_pointers/null_pointer_write.rs ... ok
tests/fail/dangling_pointers/out_of_bounds_read1.rs ... ok
tests/fail/dangling_pointers/out_of_bounds_read2.rs ... ok
tests/fail/dangling_pointers/storage_dead_dangling.rs ... ok
tests/fail/dangling_pointers/stack_temporary.rs ... ok
tests/fail/dangling_pointers/wild_pointer_deref.rs ... ok
tests/fail/data_race/alloc_read_race.rs ... ok
tests/fail/data_race/alloc_write_race.rs ... ok
tests/fail/data_race/atomic_read_na_write_race2.rs ... ok
tests/fail/data_race/atomic_write_na_read_race1.rs ... ok
tests/fail/data_race/atomic_write_na_read_race2.rs ... ok
tests/fail/data_race/atomic_read_na_write_race1.rs ... ok
tests/fail/data_race/atomic_write_na_write_race1.rs ... ok
tests/fail/data_race/atomic_write_na_write_race2.rs ... ok
tests/fail/data_race/dangling_thread_async_race.rs ... ok
tests/fail/data_race/dealloc_read_race1.rs ... ok
tests/fail/data_race/dangling_thread_race.rs ... ok
tests/fail/data_race/dealloc_read_race2.rs ... ok
tests/fail/data_race/dealloc_read_race_stack.rs ... ok
tests/fail/data_race/dealloc_write_race1.rs ... ok
tests/fail/data_race/dealloc_write_race_stack.rs ... ok
tests/fail/data_race/dealloc_write_race2.rs ... ok
tests/fail/data_race/fence_after_load.rs ... ok
tests/fail/data_race/read_write_race.rs ... ok
tests/fail/data_race/read_write_race_stack.rs ... ok
tests/fail/data_race/enable_after_join_to_main.rs ... ok
tests/fail/data_race/release_seq_race.rs ... ok
tests/fail/data_race/relax_acquire_race.rs ... ok
tests/fail/data_race/release_seq_race_same_thread.rs ... ok
tests/fail/data_race/stack_pop_race.rs ... ok
tests/fail/data_race/rmw_race.rs ... ok
tests/fail/function_calls/check_arg_abi.rs ... ok
tests/fail/data_race/write_write_race.rs ... ok
tests/fail/function_calls/check_arg_count_too_few_args.rs ... ok
tests/fail/function_calls/check_arg_count_too_many_args.rs ... ok
tests/fail/function_calls/check_arg_count_abort.rs ... ok
tests/fail/data_race/write_write_race_stack.rs ... ok
tests/fail/function_calls/check_callback_abi.rs ... ok
tests/fail/function_calls/exported_symbol_shim_clashing.rs ... ok
tests/fail/function_calls/exported_symbol_clashing.rs ... ok
tests/fail/function_calls/exported_symbol_bad_unwind1.rs ... ok
tests/fail/function_calls/exported_symbol_wrong_arguments.rs ... ok
tests/fail/function_calls/exported_symbol_wrong_type.rs ... ok
tests/fail/function_pointers/cast_box_int_to_fn_ptr.rs ... ok
tests/fail/function_pointers/cast_fn_ptr1.rs ... ok
tests/fail/function_pointers/cast_fn_ptr2.rs ... ok
tests/fail/function_pointers/cast_fn_ptr3.rs ... ok
tests/fail/function_calls/exported_symbol_abi_mismatch.rs (no_cache) ... ok
tests/fail/function_calls/exported_symbol_abi_mismatch.rs (cache) ... ok
tests/fail/function_calls/exported_symbol_abi_mismatch.rs (fn_ptr) ... ok
tests/fail/function_pointers/cast_int_to_fn_ptr.rs ... ok
tests/fail/function_pointers/cast_fn_ptr5.rs ... ok
tests/fail/function_pointers/cast_fn_ptr4.rs ... ok
tests/fail/function_pointers/deref_fn_ptr.rs ... ok
tests/fail/function_pointers/execute_memory.rs ... ok
tests/fail/function_pointers/fn_ptr_offset.rs ... ok
tests/fail/intrinsics/assume.rs ... ok
tests/fail/intrinsics/copy_null.rs ... ok
tests/fail/intrinsics/copy_overflow.rs ... ok
tests/fail/function_calls/exported_symbol_bad_unwind2.rs (extern_block) ... ok
tests/fail/function_calls/exported_symbol_bad_unwind2.rs (definition) ... ok
tests/fail/function_calls/exported_symbol_bad_unwind2.rs (both) ... ok
tests/fail/intrinsics/copy_unaligned.rs ... ok
tests/fail/intrinsics/copy_overlapping.rs ... ok
tests/fail/intrinsics/ctlz_nonzero.rs ... ok
tests/fail/intrinsics/div-by-zero.rs ... ok
tests/fail/intrinsics/exact_div1.rs ... ok
tests/fail/intrinsics/cttz_nonzero.rs ... ok
tests/fail/intrinsics/exact_div2.rs ... ok
tests/fail/intrinsics/exact_div3.rs ... ok
tests/fail/intrinsics/exact_div4.rs ... ok
tests/fail/intrinsics/float_to_int_32_infneg1.rs ... ok
tests/fail/intrinsics/float_to_int_32_inf1.rs ... ok
tests/fail/intrinsics/float_to_int_32_nan.rs ... ok
tests/fail/intrinsics/float_to_int_32_nanneg.rs ... ok
tests/fail/intrinsics/float_to_int_32_neg.rs ... ok
tests/fail/intrinsics/float_to_int_32_too_big1.rs ... ok
tests/fail/intrinsics/float_to_int_32_too_big2.rs ... ok
tests/fail/intrinsics/float_to_int_64_inf1.rs ... ok
tests/fail/intrinsics/float_to_int_32_too_small1.rs ... ok
tests/fail/intrinsics/float_to_int_64_infneg1.rs ... ok
tests/fail/intrinsics/float_to_int_64_infneg2.rs ... ok
tests/fail/intrinsics/float_to_int_64_nan.rs ... ok
tests/fail/intrinsics/float_to_int_64_neg.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_big1.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_big2.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_big3.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_big4.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_big5.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_big6.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_big7.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_small1.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_small2.rs ... ok
tests/fail/intrinsics/float_to_int_64_too_small3.rs ... ok
tests/fail/intrinsics/out_of_bounds_ptr_1.rs ... ok
tests/fail/intrinsics/out_of_bounds_ptr_2.rs ... ok
tests/fail/intrinsics/out_of_bounds_ptr_3.rs ... ok
tests/fail/intrinsics/overflowing-unchecked-rsh.rs ... ok
tests/fail/intrinsics/ptr_offset_0_plus_0.rs ... ok
tests/fail/intrinsics/ptr_offset_from_oob.rs ... ok
tests/fail/intrinsics/ptr_offset_from_unsigned_neg.rs ... ok
tests/fail/intrinsics/ptr_offset_int_plus_int.rs ... ok
tests/fail/intrinsics/ptr_offset_int_plus_ptr.rs ... ok
tests/fail/intrinsics/ptr_offset_overflow.rs ... ok
tests/fail/intrinsics/raw_eq_on_ptr.rs ... ok
tests/fail/intrinsics/ptr_offset_ptr_plus_0.rs ... ok
tests/fail/intrinsics/rem-by-zero.rs ... ok
tests/fail/intrinsics/simd-div-by-zero.rs ... ok
tests/fail/intrinsics/simd-div-overflow.rs ... ok
tests/fail/intrinsics/simd-float-to-int.rs ... ok
tests/fail/intrinsics/simd-reduce-invalid-bool.rs ... ok
tests/fail/intrinsics/simd-gather.rs ... ok
tests/fail/intrinsics/simd-scatter.rs ... ok
tests/fail/intrinsics/simd-rem-by-zero.rs ... ok
tests/fail/intrinsics/simd-select-invalid-bool.rs ... ok
tests/fail/intrinsics/simd-shr-too-far.rs ... ok
tests/fail/intrinsics/simd-select-bitmask-invalid.rs ... ok
tests/fail/intrinsics/simd-shl-too-far.rs ... ok
tests/fail/intrinsics/unchecked_add2.rs ... ok
tests/fail/intrinsics/unchecked_div1.rs ... ok
tests/fail/intrinsics/unchecked_mul1.rs ... ok
tests/fail/intrinsics/unchecked_add1.rs ... ok
tests/fail/intrinsics/unchecked_sub1.rs ... ok
tests/fail/intrinsics/unchecked_mul2.rs ... ok
tests/fail/intrinsics/unchecked_sub2.rs ... ok
tests/fail/intrinsics/uninit_uninhabited_type.rs ... ok
tests/fail/intrinsics/write_bytes_overflow.rs ... ok
tests/fail/panic/bad_miri_start_panic.rs ... ok
tests/fail/intrinsics/write_bytes_null.rs ... ok
tests/fail/intrinsics/zero_fn_ptr.rs ... ok
tests/fail/panic/no_std.rs ... ok
tests/fail/panic/panic_abort1.rs ... ok
tests/fail/panic/panic_abort2.rs ... ok
tests/fail/panic/bad_unwind.rs ... ok
tests/fail/panic/panic_abort3.rs ... ok
tests/fail/panic/panic_abort4.rs ... ok
tests/fail/provenance/pointer_partial_overwrite.rs ... ok
tests/fail/panic/unwind_panic_abort.rs ... ok
tests/fail/provenance/provenance_transmute.rs ... ok
tests/fail/provenance/ptr_int_unexposed.rs ... ok
tests/fail/provenance/ptr_invalid.rs ... ok
tests/fail/provenance/ptr_invalid_offset.rs ... ok
tests/fail/shims/shim_arg_size.rs ... ok
tests/fail/provenance/strict_provenance_cast.rs ... ok
tests/fail/stacked_borrows/alias_through_mutation.rs ... ok
tests/fail/stacked_borrows/aliasing_mut1.rs ... ok
tests/fail/stacked_borrows/aliasing_mut3.rs ... ok
tests/fail/stacked_borrows/aliasing_mut2.rs ... ok
tests/fail/stacked_borrows/aliasing_mut4.rs ... ok
tests/fail/should-pass/cpp20_rwc_syncs.rs ... ok
tests/fail/stacked_borrows/box_exclusive_violation1.rs ... ok
tests/fail/stacked_borrows/box_noalias_violation.rs ... ok
tests/fail/stacked_borrows/buggy_as_mut_slice.rs ... ok
tests/fail/stacked_borrows/buggy_split_at_mut.rs ... ok
tests/fail/stacked_borrows/deallocate_against_protector1.rs ... ok
tests/fail/stacked_borrows/disable_mut_does_not_merge_srw.rs ... ok
tests/fail/stacked_borrows/drop_in_place_retag.rs ... ok
tests/fail/stacked_borrows/drop_in_place_protector.rs ... ok
tests/fail/stacked_borrows/exposed_only_ro.rs ... ok
tests/fail/stacked_borrows/fnentry_invalidation.rs ... ok
tests/fail/stacked_borrows/fnentry_invalidation2.rs ... ok
tests/fail/stacked_borrows/illegal_dealloc1.rs ... ok
tests/fail/stacked_borrows/illegal_read1.rs ... ok
tests/fail/stacked_borrows/illegal_read2.rs ... ok
tests/fail/stacked_borrows/illegal_read3.rs ... ok
tests/fail/stacked_borrows/illegal_read5.rs ... ok
tests/fail/stacked_borrows/illegal_read6.rs ... ok
tests/fail/stacked_borrows/illegal_read4.rs ... ok
tests/fail/stacked_borrows/illegal_read7.rs ... ok
tests/fail/stacked_borrows/illegal_read8.rs ... ok
tests/fail/stacked_borrows/illegal_read_despite_exposed1.rs ... ok
tests/fail/stacked_borrows/illegal_read_despite_exposed2.rs ... ok
tests/fail/stacked_borrows/illegal_write1.rs ... ok
tests/fail/stacked_borrows/illegal_write2.rs ... ok
tests/fail/stacked_borrows/illegal_write3.rs ... ok
tests/fail/stacked_borrows/illegal_write4.rs ... ok
tests/fail/stacked_borrows/illegal_write5.rs ... ok
tests/fail/stacked_borrows/illegal_write_despite_exposed1.rs ... ok
tests/fail/stacked_borrows/illegal_write6.rs ... ok
tests/fail/stacked_borrows/interior_mut1.rs ... ok
tests/fail/stacked_borrows/invalidate_against_protector1.rs ... ok
tests/fail/stacked_borrows/interior_mut2.rs ... ok
tests/fail/stacked_borrows/invalidate_against_protector2.rs ... ok
tests/fail/stacked_borrows/invalidate_against_protector3.rs ... ok
tests/fail/stacked_borrows/issue-miri-1050-1.rs ... ok
tests/fail/stacked_borrows/issue-miri-1050-2.rs ... ok
tests/fail/stacked_borrows/load_invalid_mut.rs ... ok
tests/fail/stacked_borrows/load_invalid_shr.rs ... ok
tests/fail/stacked_borrows/mut_exclusive_violation1.rs ... ok
tests/fail/stacked_borrows/newtype_pair_retagging.rs ... ok
tests/fail/stacked_borrows/newtype_retagging.rs ... ok
tests/fail/stacked_borrows/mut_exclusive_violation2.rs ... ok
tests/fail/stacked_borrows/outdated_local.rs ... ok
tests/fail/stacked_borrows/pass_invalid_mut.rs ... ok
tests/fail/stacked_borrows/pass_invalid_shr.rs ... ok
tests/fail/stacked_borrows/pointer_smuggling.rs ... ok
tests/fail/stacked_borrows/raw_tracking.rs ... ok
tests/fail/stacked_borrows/return_invalid_mut.rs ... ok
tests/fail/stacked_borrows/return_invalid_mut_tuple.rs ... ok
tests/fail/stacked_borrows/return_invalid_mut_option.rs ... ok
tests/fail/stacked_borrows/retag_data_race_read.rs ... ok
tests/fail/stacked_borrows/retag_data_race_write.rs ... ok
tests/fail/stacked_borrows/return_invalid_shr.rs ... ok
tests/fail/stacked_borrows/return_invalid_shr_option.rs ... ok
tests/fail/stacked_borrows/shared_rw_borrows_are_weak1.rs ... ok
tests/fail/stacked_borrows/return_invalid_shr_tuple.rs ... ok
tests/fail/stacked_borrows/shared_rw_borrows_are_weak2.rs ... ok
tests/fail/stacked_borrows/static_memory_modification.rs ... ok
tests/fail/stacked_borrows/transmute-is-no-escape.rs ... ok
tests/fail/stacked_borrows/track_caller.rs ... ok
tests/fail/stacked_borrows/shr_frozen_violation1.rs ... ok
tests/fail/stacked_borrows/unescaped_local.rs ... ok
tests/fail/stacked_borrows/unescaped_static.rs ... ok
tests/fail/tree-borrows/alternate-read-write.rs ... ok
tests/fail/stacked_borrows/zst_slice.rs ... ok
tests/fail/tree-borrows/outside-range.rs ... ok
tests/fail/tree-borrows/fragile-data-race.rs ... ok
tests/fail/tree-borrows/read-to-local.rs ... ok
tests/fail/tree-borrows/write-during-2phase.rs ... ok
tests/fail/unaligned_pointers/alignment.rs ... ok
tests/fail/tree-borrows/retag-data-race.rs ... ok
tests/fail/unaligned_pointers/atomic_unaligned.rs ... ok
tests/fail/unaligned_pointers/drop_in_place.rs ... ok
tests/fail/unaligned_pointers/dyn_alignment.rs ... ok
tests/fail/unaligned_pointers/intptrcast_alignment_check.rs ... ok
tests/fail/unaligned_pointers/unaligned_ptr1.rs ... ok
tests/fail/unaligned_pointers/reference_to_packed.rs ... ok
tests/fail/unaligned_pointers/unaligned_ptr2.rs ... ok
tests/fail/unaligned_pointers/unaligned_ptr3.rs ... ok
tests/fail/unaligned_pointers/unaligned_ptr4.rs ... ok
tests/fail/unaligned_pointers/unaligned_ptr_addr_of.rs ... ok
tests/fail/unaligned_pointers/unaligned_ptr_zst.rs ... ok
tests/fail/validity/cast_fn_ptr1.rs ... ok
tests/fail/validity/cast_fn_ptr2.rs ... ok
tests/fail/validity/dangling_ref1.rs ... ok
tests/fail/validity/dangling_ref2.rs ... ok
tests/fail/validity/dangling_ref3.rs ... ok
tests/fail/validity/invalid_bool.rs ... ok
tests/fail/validity/invalid_bool_uninit.rs ... ok
tests/fail/validity/invalid_char.rs ... ok
tests/fail/validity/invalid_char_uninit.rs ... ok
tests/fail/validity/invalid_fnptr_null.rs ... ok
tests/fail/validity/invalid_enum_tag.rs ... ok
tests/fail/validity/invalid_fnptr_uninit.rs ... ok
tests/fail/validity/nonzero.rs ... ok
tests/fail/validity/invalid_wide_raw.rs ... ok
tests/fail/validity/ref_to_uninhabited1.rs ... ok
tests/fail/validity/ref_to_uninhabited2.rs ... ok
tests/fail/validity/too-big-unsized.rs ... ok
tests/fail/validity/too-big-slice.rs ... ok
tests/fail/validity/uninit_integer.rs ... ok
tests/fail/validity/uninit_float.rs ... ok
tests/fail/validity/transmute_through_ptr.rs ... ok
tests/fail/validity/uninit_raw_ptr.rs ... ok
tests/fail/shims/backtrace/bad-backtrace-decl.rs ... ok
tests/fail/shims/backtrace/bad-backtrace-flags.rs ... ok
tests/fail/weak_memory/racing_mixed_size_read.rs ... ok
tests/fail/weak_memory/racing_mixed_size.rs ... ok
tests/fail/shims/backtrace/bad-backtrace-ptr.rs ... ok
tests/fail/shims/backtrace/bad-backtrace-resolve-flags.rs ... ok
tests/fail/shims/backtrace/bad-backtrace-resolve-names-flags.rs ... ok
tests/fail/shims/backtrace/bad-backtrace-size-flags.rs ... ok
tests/fail/shims/fs/close_stdout.rs ... ok
tests/fail/shims/fs/isolated_file.rs ... ok
tests/fail/shims/fs/mkstemp_immutable_arg.rs ... ok
tests/fail/shims/fs/isolated_stdin.rs ... ok
tests/fail/shims/fs/read_from_stdout.rs ... ok
tests/fail/shims/fs/unix_open_missing_required_mode.rs ... ok
tests/fail/shims/fs/write_to_stdin.rs ... ok
tests/fail/shims/sync/libc_pthread_condattr_double_destroy.rs ... ok
tests/fail/shims/sync/libc_pthread_cond_double_destroy.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_NULL_deadlock.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_default_deadlock.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_deadlock.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_double_destroy.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_normal_deadlock.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_destroy_locked.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_normal_unlock_unlocked.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_destroy_read_locked.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_double_destroy.rs ... ok
tests/fail/shims/sync/libc_pthread_mutex_wrong_owner.rs ... ok
tests/fail/shims/sync/libc_pthread_mutexattr_double_destroy.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_destroy_write_locked.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_read_write_deadlock_single_thread.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_unlock_unlocked.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_write_read_deadlock_single_thread.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_read_wrong_owner.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_write_read_deadlock.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_write_write_deadlock_single_thread.rs ... ok
tests/fail/tree-borrows/reserved/cell-protected-write.rs ... ok
tests/fail/tree-borrows/reserved/int-protected-write.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_write_write_deadlock.rs ... ok
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... ok

test result: ok. 377 tests passed, 3 ignored, 0 filtered out

## Running ui tests in tests/extern-so/pass against miri for x86_64-unknown-linux-gnu
The application panicked (crashed).
Message:  failed to generate shared object file for testing external C function calls: Os { code: 2, kind: NotFound, message: "No such file or directory" }
Location: src/tools/miri/tests/compiletest.rs:39

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
error: test failed, to rerun pass `--test compiletest`
Build completed unsuccessfully in 0:01:14
