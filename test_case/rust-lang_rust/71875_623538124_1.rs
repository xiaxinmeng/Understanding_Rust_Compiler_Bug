\n\nThis error occurs when the compiler is unable to infer the concrete type of a\nvariable. It can occur in several cases, the most common being a mismatch\nbetween two types: the type the author explicitly assigned, and the type the\ncompiler inferred.\n"},"level":"error","spans":[{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":104,"byte_end":106,"line_start":5,"line_end":5,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    42","highlight_start":5,"highlight_end":7}],"label":"expected type parameter `u32`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":78,"byte_end":81,"line_start":4,"line_end":4,"column_start":8,"column_end":11,"is_primary":false,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":8,"highlight_end":11}],"label":"this type parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":94,"byte_end":97,"line_start":4,"line_end":4,"column_start":24,"column_end":27,"is_primary":false,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":24,"highlight_end":27}],"label":"expected `u32` because of return type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type parameter `u32`\n             found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> tests/ui/builtin-type-shadow.rs:5:5\n   |\nLL | fn foo<u32>(a: u32) -> u32 {\n   |        ---             --- expected `u32` because of return type\n   |        |\n   |        this type parameter\nLL |     42\n   |     ^^ expected type parameter `u32`, found integer\n   |\n   = note: expected type parameter `u32`\n                        found type `{integer}`\n\n"}
{"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}

------------------------------------------

---
   Compiling rustc-ap-rustc_errors v654.0.0
   Compiling rustc-ap-rustc_feature v654.0.0
   Compiling jsonrpc-core-client v14.0.5
   Compiling rustc-ap-rustc_ast_pretty v654.0.0
   Compiling rls-ipc v0.1.0 (/checkout/src/tools/rls/rls-ipc)
   Compiling rls-rustc v0.6.0 (/checkout/src/tools/rls/rls-rustc)
   Compiling rustc-ap-rustc_parse v654.0.0
   Compiling rustc-ap-rustc_attr v654.0.0
   Compiling racer v2.1.33
---
test string::test::should_break_forward ... ok
test string::test::should_break_on_punctuation ... ok
test string::test::significant_whitespaces ... ok
test string::test::should_break_on_whitespace ... ok
test syntux::session::tests::emitter::handles_fatal_parse_error_in_ignored_file ... ok
test syntux::session::tests::emitter::handles_mix_of_recoverable_parse_error ... ok
test syntux::session::tests::emitter::handles_recoverable_parse_error_in_ignored_file ... ok
test syntux::session::tests::emitter::handles_recoverable_parse_error_in_non_ignored_file ... ok
test test::coverage_tests ... ok
test test::format_lines_errors_are_reported ... ok
test test::format_lines_errors_are_reported_with_tabs ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok
---
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-df3a980c51592be8
## Running run-pass tests in tests/run-pass against miri for target x86_64-unknown-linux-gnu
   Compiler flags: --edition 2018 -Astable-features --sysroot /home/user/.cache/miri/HOST
running 190 tests
test [ui] run-pass/args.rs ... ok
test [ui] run-pass/arrays.rs ... ok
test [ui] run-pass/associated-const.rs ... ok
---
test [ui] run-pass/closure-field-ty.rs ... ok
test [ui] run-pass/closures.rs ... ok
test [ui] run-pass/coerce_non_capture_closure_to_fn_ptr.rs ... ok
test [ui] run-pass/coercions.rs ... ok
test [ui] run-pass/concurrency/locks.rs ... ok
test [ui] run-pass/concurrency/simple.rs ... ok
test [ui] run-pass/concurrency/thread_locals.rs ... ok
test [ui] run-pass/concurrency/tls_lib_drop_single_thread.rs ... ok
test [ui] run-pass/const-vec-of-fns.rs ... ok
test [ui] run-pass/concurrency/tls_lib_drop.rs ... ok
test [ui] run-pass/current_dir.rs ... ok
test [ui] run-pass/deriving-associated-types.rs ... ok
test [ui] run-pass/disable-alignment-check.rs ... ok
test [ui] run-pass/drop_empty_slice.rs ... ok
---

test result: ok. 190 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

## Running compile-fail tests in tests/compile-fail against miri for target x86_64-unknown-linux-gnu
   Compiler flags: --edition 2018 -Astable-features --sysroot /home/user/.cache/miri/HOST
running 220 tests
test [compile-fail] compile-fail/alloc/deallocate-bad-alignment.rs ... ok
test [compile-fail] compile-fail/abort-terminator.rs ... ok
test [compile-fail] compile-fail/alloc/deallocate-bad-size.rs ... ok
test [compile-fail] compile-fail/alloc/deallocate-bad-size.rs ... ok
test [compile-fail] compile-fail/alloc/deallocate-twice.rs ... ok
test [compile-fail] compile-fail/alloc/reallocate-bad-size.rs ... ok
test [compile-fail] compile-fail/alloc/reallocate-change-alloc.rs ... ok
test [compile-fail] compile-fail/alloc/reallocate-dangling.rs ... ok
test [compile-fail] compile-fail/alloc/stack_free.rs ... ok
test [compile-fail] compile-fail/concurrency/libc_pthread_create_main_terminate.rs ... ok
test [compile-fail] compile-fail/concurrency/libc_pthread_join_detached.rs ... ok
test [compile-fail] compile-fail/concurrency/libc_pthread_join_joined.rs ... ok
test [compile-fail] compile-fail/concurrency/libc_pthread_join_main.rs ... ok
test [compile-fail] compile-fail/concurrency/libc_pthread_join_multiple.rs ... ok
test [compile-fail] compile-fail/concurrency/thread-spawn.rs ... ignored
test [compile-fail] compile-fail/concurrency/libc_pthread_join_self.rs ... ok
test [compile-fail] compile-fail/dangling_pointers/dangling_zst_deref.rs ... ok
test [compile-fail] compile-fail/dangling_pointers/deref-invalid-ptr.rs ... ok
test [compile-fail] compile-fail/dangling_pointers/deref-partially-dangling.rs ... ok
test [compile-fail] compile-fail/dangling_pointers/dyn_size.rs ... ok
---
test [compile-fail] compile-fail/intrinsics/exact_div1.rs ... ok
test [compile-fail] compile-fail/intrinsics/exact_div2.rs ... ok
test [compile-fail] compile-fail/intrinsics/exact_div3.rs ... ok
test [compile-fail] compile-fail/intrinsics/exact_div4.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_inf1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_infneg1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_nanneg.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_nan.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_neg.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_too_big1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_too_big2.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_32_too_small1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_infneg1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_inf1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_infneg2.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_nan.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_neg.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_big1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_big2.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_big3.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_big5.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_big4.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_big7.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_big6.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_small1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_small2.rs ... ok
test [compile-fail] compile-fail/intrinsics/out_of_bounds_ptr_1.rs ... ok
test [compile-fail] compile-fail/intrinsics/float_to_int_64_too_small3.rs ... ok
test [compile-fail] compile-fail/intrinsics/overflowing-unchecked-rsh.rs ... ok
test [compile-fail] compile-fail/intrinsics/out_of_bounds_ptr_2.rs ... ok
test [compile-fail] compile-fail/intrinsics/ptr_offset_0_plus_0.rs ... ok
test [compile-fail] compile-fail/intrinsics/ptr_offset_int_plus_int.rs ... ok
test [compile-fail] compile-fail/intrinsics/ptr_offset_overflow.rs ... ok
test [compile-fail] compile-fail/intrinsics/ptr_offset_int_plus_ptr.rs ... ok
test [compile-fail] compile-fail/intrinsics/ptr_offset_ptr_plus_0.rs ... ok
test [compile-fail] compile-fail/intrinsics/unchecked_add2.rs ... ok
test [compile-fail] compile-fail/intrinsics/unchecked_div1.rs ... ok
test [compile-fail] compile-fail/intrinsics/unchecked_mul1.rs ... ok
test [compile-fail] compile-fail/intrinsics/unchecked_mul2.rs ... ok
---
test [compile-fail] compile-fail/panic/double_panic.rs ... ok
test [compile-fail] compile-fail/rc_as_ptr.rs ... ok
test [compile-fail] compile-fail/rustc-error.rs ... ok
test [compile-fail] compile-fail/reading_half_a_pointer.rs ... ok
test [compile-fail] compile-fail/shim_arg_size.rs ... ok
test [compile-fail] compile-fail/stacked_borrows/alias_through_mutation.rs ... ok
test [compile-fail] compile-fail/stacked_borrows/aliasing_mut1.rs ... ok
test [compile-fail] compile-fail/stacked_borrows/aliasing_mut2.rs ... ok
test [compile-fail] compile-fail/stacked_borrows/aliasing_mut3.rs ... ok
---
test [compile-fail] compile-fail/static_memory_modification1.rs ... ok
test [compile-fail] compile-fail/static_memory_modification3.rs ... ok
test [compile-fail] compile-fail/static_memory_modification2.rs ... ok
test [compile-fail] compile-fail/storage_dead_dangling.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_mutex_deadlock.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_mutex_destroy_locked.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_mutex_normal_deadlock.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_mutex_normal_unlock_unlocked.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_destroy_read_locked.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_mutex_wrong_owner.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_read_write_deadlock.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_destroy_write_locked.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_unlock_unlocked.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_write_read_deadlock.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_write_read_deadlock_single_thread.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_write_write_deadlock.rs ... ok
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_write_write_deadlock_single_thread.rs ... ok
test [compile-fail] compile-fail/transmute_fat1.rs ... ok
test [compile-fail] compile-fail/unaligned_pointers/alignment.rs ... ok
test [compile-fail] compile-fail/unaligned_pointers/atomic_unaligned.rs ... ok
test [compile-fail] compile-fail/unaligned_pointers/dyn_alignment.rs ... ok
---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
proper steps.
Build completed unsuccessfully in 0:00:01
== clock drift check ==
  local time: Mon May  4 15:37:00 UTC 2020
  network time: Mon, 04 May 2020 15:37:00 GMT
  network time: Mon, 04 May 2020 15:37:00 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71875/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71875/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3508) (python)
##[section]Finishing: Finalize Job
