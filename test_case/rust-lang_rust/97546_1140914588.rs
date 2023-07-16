plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 855fc022fe879f4e3493a024f9c6b981d6317612 and 0eb426d850f938b6c2c34f1d48b5b0c797a91298
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    = note: `#[warn(rustc::pass_by_value)]` on by default

warning: `miri` (lib) generated 1 warning
   Compiling crossbeam v0.8.1
   Compiling ui_test v0.1.0 (/checkout/src/tools/miri/ui_test)
warning: `miri` (lib test) generated 1 warning (1 duplicate)
    Finished release [optimized] target(s) in 10.72s
     Running unittests src/lib.rs (build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/miri-534621c21fc88543)

---

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/compiletest.rs (build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-4581f3b332bd4b76)
## Running ui tests in tests/run-pass against miri for host
   Compiler flags: ["--edition", "2018", "-Astable-features", "--sysroot", "/home/user/.cache/miri/HOST", "-Zui-testing"]
tests/run-pass/move-arg-3-unique.rs ... ok
tests/run-pass/assume_bug.rs ... ok
tests/run-pass/tuple_like_enum_variant_constructor.rs ... ok
tests/run-pass/regions-mock-trans.rs ... ok
---
tests/run-pass/backtrace-std.rs ... ok
tests/run-pass/concurrency/sync.rs ... ok
tests/run-pass/concurrency/libc_pthread_cond.rs ... ok

test result: ok. 221 tests passed, 4 ignored
## Running ui tests in tests/run-fail against miri for host
## Running ui tests in tests/run-fail against miri for host
   Compiler flags: ["--edition", "2018", "-Astable-features", "--sysroot", "/home/user/.cache/miri/HOST", "-Zui-testing"]
tests/run-fail/panic/overflowing-lsh-neg.rs ... ok
tests/run-fail/panic/panic3.rs ... ok
tests/run-fail/panic/unsupported_foreign_function.rs ... ok
tests/run-fail/panic/unsupported_syscall.rs ... ok
tests/run-fail/panic/overflowing-rsh-1.rs ... ok
tests/run-fail/panic/overflowing-rsh-2.rs ... ok
tests/run-fail/panic/div-by-zero-2.rs ... ok
tests/run-fail/panic/panic2.rs ... ok
tests/run-fail/transmute_fat2.rs ... ok
tests/run-fail/panic/panic4.rs ... ok
tests/run-fail/function_calls/exported_symbol_good_unwind.rs ... ok
tests/run-fail/panic/panic1.rs ... ok

test result: ok. 12 tests passed, 0 ignored
## Running ui tests in tests/compile-fail against miri for host
## Running ui tests in tests/compile-fail against miri for host
   Compiler flags: ["--edition", "2018", "-Astable-features", "--sysroot", "/home/user/.cache/miri/HOST", "-Zui-testing"]
tests/compile-fail/invalid_bool.rs ... ok
tests/compile-fail/static_memory_modification3.rs ... ok
tests/compile-fail/never_say_never.rs ... ok
tests/compile-fail/fast_math_first.rs ... ok
---
tests/compile-fail/function_calls/check_callback_abi.rs ... ok
tests/compile-fail/function_calls/exported_symbol_wrong_arguments.rs ... ok
tests/compile-fail/function_calls/check_arg_count_too_many_args.rs ... ok
tests/compile-fail/function_calls/exported_symbol_wrong_type.rs ... ok
tests/compile-fail/function_calls/exported_symbol_abi_mismatch.rs (revision `no_cache`) ... ok
tests/compile-fail/function_calls/exported_symbol_clashing.rs ... ok
tests/compile-fail/function_calls/check_arg_abi.rs ... ok
tests/compile-fail/function_calls/exported_symbol_shim_clashing.rs ... ok
tests/compile-fail/panic/bad_miri_start_panic.rs ... ok
tests/compile-fail/panic/bad_miri_start_panic.rs ... ok
tests/compile-fail/panic/unwind_panic_abort.rs ... ok
tests/compile-fail/function_calls/exported_symbol_bad_unwind2.rs (revision `extern_block`) ... ok
tests/compile-fail/panic/panic_abort1.rs ... ok
tests/compile-fail/panic/panic_abort1.rs ... ok
tests/compile-fail/function_calls/exported_symbol_abi_mismatch.rs (revision `cache`) ... ok
tests/compile-fail/fs/write_to_stdin.rs ... ok
tests/compile-fail/fs/close_stdout.rs ... ok
tests/compile-fail/fs/read_from_stdout.rs ... ok
tests/compile-fail/fs/unix_open_missing_required_mode.rs ... ok
tests/compile-fail/fs/unix_open_missing_required_mode.rs ... ok
tests/compile-fail/panic/bad_unwind.rs ... ok
tests/compile-fail/panic/panic_abort3.rs ... ok
tests/compile-fail/panic/panic_abort2.rs ... ok
tests/compile-fail/function_calls/exported_symbol_abi_mismatch.rs (revision `fn_ptr`) ... ok
tests/compile-fail/provenance/strict_provenance_transmute.rs ... ok
tests/compile-fail/fs/isolated_file.rs ... ok
tests/compile-fail/provenance/ptr_int_unexposed.rs ... ok
tests/compile-fail/provenance/ptr_legacy_provenance.rs ... ok
tests/compile-fail/provenance/ptr_legacy_provenance.rs ... ok
tests/compile-fail/provenance/strict-provenance-offset.rs ... ok
tests/compile-fail/provenance/ptr_invalid.rs ... ok
tests/compile-fail/function_calls/exported_symbol_bad_unwind2.rs (revision `definition`) ... ok
tests/compile-fail/unaligned_pointers/reference_to_packed.rs ... ok
tests/compile-fail/unaligned_pointers/unaligned_ptr_zst.rs ... ok
tests/compile-fail/unaligned_pointers/unaligned_ptr2.rs ... ok
tests/compile-fail/unaligned_pointers/unaligned_ptr1.rs ... ok
---
tests/compile-fail/dangling_pointers/null_pointer_deref_zst.rs ... ok
tests/compile-fail/dangling_pointers/maybe_null_pointer_write_zst.rs ... ok
tests/compile-fail/dangling_pointers/null_pointer_write_zst.rs ... ok
tests/compile-fail/dangling_pointers/maybe_null_pointer_deref_zst.rs ... ok
tests/compile-fail/function_calls/exported_symbol_bad_unwind2.rs (revision `both`) ... ok
tests/compile-fail/alloc/no_global_allocator.rs ... ok
tests/compile-fail/dangling_pointers/storage_dead_dangling.rs ... ok
tests/compile-fail/dangling_pointers/out_of_bounds_read2.rs ... ok
tests/compile-fail/dangling_pointers/deref-invalid-ptr.rs ... ok
---
tests/compile-fail/concurrency/libc_pthread_join_self.rs ... ok
tests/compile-fail/concurrency/unwind_top_of_stack.rs ... ok
tests/compile-fail/panic/double_panic.rs ... ok

test result: ok. 336 tests passed, 1 ignored
warning: passing `TyCtxt<'_>` by reference
   --> src/tools/miri/src/helpers.rs:861:30
    |
861 | pub fn get_local_crates(tcx: &TyCtxt<'_>) -> Vec<CrateNum> {
---

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/compiletest.rs (build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-4581f3b332bd4b76)
## Running ui tests in tests/run-pass against miri for host
   Compiler flags: ["--edition", "2018", "-Astable-features", "--sysroot", "/home/user/.cache/miri/HOST", "-O", "-Zmir-opt-level=4", "-Zui-testing"]
tests/run-pass/assume_bug.rs ... ok
tests/run-pass/regions-mock-trans.rs ... ok
tests/run-pass/move-arg-3-unique.rs ... ok
tests/run-pass/tuple_like_enum_variant_constructor.rs ... ok
---
tests/run-pass/backtrace-std.rs ... ok
tests/run-pass/concurrency/sync.rs ... ok
tests/run-pass/concurrency/libc_pthread_cond.rs ... ok

test result: ok. 221 tests passed, 4 ignored
## Running ui tests in tests/run-fail against miri for host
## Running ui tests in tests/run-fail against miri for host
   Compiler flags: ["--edition", "2018", "-Astable-features", "--sysroot", "/home/user/.cache/miri/HOST", "-O", "-Zmir-opt-level=4", "-Zui-testing"]
tests/run-fail/panic/unsupported_foreign_function.rs ... ok
tests/run-fail/panic/div-by-zero-2.rs ... ok
tests/run-fail/panic/overflowing-lsh-neg.rs ... ok
tests/run-fail/panic/unsupported_syscall.rs ... ok
tests/run-fail/panic/overflowing-rsh-2.rs ... ok
tests/run-fail/panic/overflowing-rsh-1.rs ... ok
tests/run-fail/panic/panic3.rs ... ok
tests/run-fail/transmute_fat2.rs ... ok
tests/run-fail/function_calls/exported_symbol_good_unwind.rs ... ok
tests/run-fail/panic/panic4.rs ... ok
tests/run-fail/panic/panic2.rs ... ok
tests/run-fail/panic/panic1.rs ... ok

test result: ok. 12 tests passed, 0 ignored
## Running ui tests in tests/compile-fail against miri for host
## Running ui tests in tests/compile-fail against miri for host
   Compiler flags: ["--edition", "2018", "-Astable-features", "--sysroot", "/home/user/.cache/miri/HOST", "-O", "-Zmir-opt-level=4", "-Zui-testing"]
tests/compile-fail/static_memory_modification3.rs ... ok
tests/compile-fail/erroneous_const.rs ... ok
tests/compile-fail/unsupported_signal.rs ... ok
tests/compile-fail/breakpoint.rs ... ok
---
tests/compile-fail/intrinsics/float_to_int_64_inf1.rs ... ok
tests/compile-fail/function_calls/check_callback_abi.rs ... ok
tests/compile-fail/intrinsics/out_of_bounds_ptr_2.rs ... ok
tests/compile-fail/function_calls/exported_symbol_clashing.rs ... ok
tests/compile-fail/function_calls/exported_symbol_abi_mismatch.rs (revision `no_cache`) ... ok
tests/compile-fail/function_calls/check_arg_abi.rs ... ok
tests/compile-fail/function_calls/check_arg_count_too_few_args.rs ... ok
tests/compile-fail/function_calls/exported_symbol_shim_clashing.rs ... ok
tests/compile-fail/panic/bad_miri_start_panic.rs ... ok
tests/compile-fail/panic/bad_miri_start_panic.rs ... ok
tests/compile-fail/panic/unwind_panic_abort.rs ... ok
tests/compile-fail/function_calls/exported_symbol_abi_mismatch.rs (revision `cache`) ... ok
tests/compile-fail/function_calls/exported_symbol_bad_unwind1.rs ... ok
tests/compile-fail/function_calls/exported_symbol_bad_unwind2.rs (revision `extern_block`) ... ok
tests/compile-fail/panic/panic_abort4.rs ... ok
tests/compile-fail/fs/write_to_stdin.rs ... ok
tests/compile-fail/fs/close_stdout.rs ... ok
tests/compile-fail/panic/bad_unwind.rs ... ok
tests/compile-fail/panic/bad_unwind.rs ... ok
tests/compile-fail/function_calls/exported_symbol_abi_mismatch.rs (revision `fn_ptr`) ... ok
tests/compile-fail/panic/panic_abort3.rs ... ok
tests/compile-fail/panic/panic_abort2.rs ... ok
tests/compile-fail/fs/isolated_stdin.rs ... ok
tests/compile-fail/fs/unix_open_missing_required_mode.rs ... ok
tests/compile-fail/fs/unix_open_missing_required_mode.rs ... ok
tests/compile-fail/fs/isolated_file.rs ... ok
tests/compile-fail/provenance/ptr_legacy_provenance.rs ... ok
tests/compile-fail/provenance/strict-provenance-offset.rs ... ok
tests/compile-fail/provenance/strict_provenance_transmute.rs ... ok
tests/compile-fail/provenance/ptr_int_unexposed.rs ... ok
tests/compile-fail/provenance/ptr_invalid.rs ... FAILED
tests/compile-fail/function_calls/exported_symbol_bad_unwind2.rs (revision `definition`) ... ok
tests/compile-fail/unaligned_pointers/unaligned_ptr_zst.rs ... ok
tests/compile-fail/unaligned_pointers/unaligned_ptr2.rs ... FAILED
tests/compile-fail/unaligned_pointers/unaligned_ptr1.rs ... FAILED
tests/compile-fail/unaligned_pointers/reference_to_packed.rs ... FAILED
---
tests/compile-fail/dangling_pointers/null_pointer_deref.rs ... ok
tests/compile-fail/dangling_pointers/null_pointer_deref_zst.rs ... ok
tests/compile-fail/dangling_pointers/dangling_pointer_deref.rs ... ok
tests/compile-fail/dangling_pointers/dangling_zst_deref.rs ... ok
tests/compile-fail/function_calls/exported_symbol_bad_unwind2.rs (revision `both`) ... ok
tests/compile-fail/dangling_pointers/maybe_null_pointer_deref_zst.rs ... ok
tests/compile-fail/alloc/no_global_allocator.rs ... ok
tests/compile-fail/dangling_pointers/wild_pointer_deref.rs ... ok
tests/compile-fail/dangling_pointers/out_of_bounds_read2.rs ... ok
---
tests/compile-fail/concurrency/unwind_top_of_stack.rs ... ok
tests/compile-fail/panic/double_panic.rs ... ok

tests/compile-fail/invalid_bool.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/invalid_bool.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation"
Fail got exit status: 0


`interpreting an invalid 8-bit value as a bool` not found in stderr output
expected because of pattern here: tests/compile-fail/invalid_bool.rs:7
actual stderr:




tests/compile-fail/extern_static.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/extern_static.rs"
Fail got exit status: 0


`is not supported by Miri` not found in stderr output
expected because of pattern here: tests/compile-fail/extern_static.rs:7
actual stderr:




tests/compile-fail/reading_half_a_pointer.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/reading_half_a_pointer.rs"
Fail got exit status: 0


`unable to turn pointer into raw bytes` not found in stderr output
expected because of pattern here: tests/compile-fail/reading_half_a_pointer.rs:26
actual stderr:




tests/compile-fail/environ-gets-deallocated.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/environ-gets-deallocated.rs"
Fail got exit status: 0


`dereferenced after this allocation got freed` not found in stderr output
expected because of pattern here: tests/compile-fail/environ-gets-deallocated.rs:22
actual stderr:




tests/compile-fail/pointer_partial_read.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/pointer_partial_read.rs"
Fail got exit status: 0


`unable to turn pointer into raw bytes` not found in stderr output
expected because of pattern here: tests/compile-fail/pointer_partial_read.rs:7
actual stderr:




tests/compile-fail/invalid_char.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/invalid_char.rs" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation"
Fail got exit status: 0


`interpreting an invalid 32-bit value as a char` not found in stderr output
expected because of pattern here: tests/compile-fail/invalid_char.rs:8
actual stderr:




tests/compile-fail/zst1.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/zst1.rs"
Fail got exit status: 0


`out-of-bounds` not found in stderr output
expected because of pattern here: tests/compile-fail/zst1.rs:3
actual stderr:




tests/compile-fail/validity/invalid_fnptr_uninit.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/validity/invalid_fnptr_uninit.rs"
Fail got exit status: 0

`encountered uninitialized bytes` not found in stderr output
expected because of pattern here: tests/compile-fail/validity/invalid_fnptr_uninit.rs:8
expected because of pattern here: tests/compile-fail/validity/invalid_fnptr_uninit.rs:8

actual stderr:



tests/compile-fail/type-too-large.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/type-too-large.rs"
Fail got exit status: 0


`post-monomorphization error` not found in stderr output
expected because of pattern here: tests/compile-fail/type-too-large.rs:4
actual stderr:




tests/compile-fail/validity/invalid_char_uninit.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/validity/invalid_char_uninit.rs"
Fail got exit status: 0


`encountered uninitialized bytes, but expected a valid unicode scalar value` not found in stderr output
expected because of pattern here: tests/compile-fail/validity/invalid_char_uninit.rs:8
actual stderr:




tests/compile-fail/validity/invalid_enum_tag_256variants_uninit.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/validity/invalid_enum_tag_256variants_uninit.rs" "-Zmiri-allow-uninit-numbers"
Fail got exit status: 0


`type validation failed at .<enum-tag>: encountered uninitialized bytes, but expected a valid enum tag` not found in stderr output
expected because of pattern here: tests/compile-fail/validity/invalid_enum_tag_256variants_uninit.rs:269
actual stderr:




tests/compile-fail/validity/invalid_bool_uninit.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/validity/invalid_bool_uninit.rs"
Fail got exit status: 0


`encountered uninitialized bytes, but expected a boolean` not found in stderr output
expected because of pattern here: tests/compile-fail/validity/invalid_bool_uninit.rs:8
actual stderr:




tests/compile-fail/provenance/ptr_invalid.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/provenance/ptr_invalid.rs"
Fail got exit status: 0


`is not a valid pointer` not found in stderr output
expected because of pattern here: tests/compile-fail/provenance/ptr_invalid.rs:7
actual stderr:




tests/compile-fail/unaligned_pointers/unaligned_ptr2.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/unaligned_pointers/unaligned_ptr2.rs" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows"
Fail got exit status: 0


`memory with alignment 1, but alignment 4 is required` not found in stderr output
expected because of pattern here: tests/compile-fail/unaligned_pointers/unaligned_ptr2.rs:10
actual stderr:




tests/compile-fail/unaligned_pointers/unaligned_ptr1.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/unaligned_pointers/unaligned_ptr1.rs" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows"
Fail got exit status: 0


`memory with alignment 2, but alignment 4 is required` not found in stderr output
expected because of pattern here: tests/compile-fail/unaligned_pointers/unaligned_ptr1.rs:8
actual stderr:




tests/compile-fail/unaligned_pointers/reference_to_packed.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/unaligned_pointers/reference_to_packed.rs" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows"
Fail got exit status: 0


`alignment 4 is required` not found in stderr output
expected because of pattern here: tests/compile-fail/unaligned_pointers/reference_to_packed.rs:18
actual stderr:




tests/compile-fail/unaligned_pointers/unaligned_ptr4.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/unaligned_pointers/unaligned_ptr4.rs" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows"
Fail got exit status: 0


`but alignment` not found in stderr output
expected because of pattern here: tests/compile-fail/unaligned_pointers/unaligned_ptr4.rs:9
actual stderr:




tests/compile-fail/unaligned_pointers/unaligned_ptr_addr_of.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/unaligned_pointers/unaligned_ptr_addr_of.rs" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows"
Fail got exit status: 0


`memory with alignment 2, but alignment 4 is required` not found in stderr output
expected because of pattern here: tests/compile-fail/unaligned_pointers/unaligned_ptr_addr_of.rs:10
actual stderr:




tests/compile-fail/unaligned_pointers/unaligned_ptr3.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/unaligned_pointers/unaligned_ptr3.rs" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows"
Fail got exit status: 0


`but alignment` not found in stderr output
expected because of pattern here: tests/compile-fail/unaligned_pointers/unaligned_ptr3.rs:9
actual stderr:




tests/compile-fail/alloc/reallocate-change-alloc.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/alloc/reallocate-change-alloc.rs"

`dereferenced after this allocation got freed` not found in stderr output
expected because of pattern here: tests/compile-fail/alloc/reallocate-change-alloc.rs:6
actual stderr:
actual stderr:
The following memory was leaked: alloc1677 (Rust heap, size: 1, align: 1) {
    __                                              │ ░

error: the evaluated program leaked memory


note: pass `-Zmiri-ignore-leaks` to disable this check
error: aborting due to previous error





tests/compile-fail/stacked_borrows/illegal_read2.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_read2.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_read2.rs:9
actual stderr:




tests/compile-fail/stacked_borrows/illegal_read6.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_read6.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_read6.rs:6
actual stderr:




tests/compile-fail/stacked_borrows/illegal_write4.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_write4.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_write4.rs:11
actual stderr:




tests/compile-fail/stacked_borrows/return_invalid_mut_option.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/return_invalid_mut_option.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/return_invalid_mut_option.rs:12
actual stderr:




tests/compile-fail/stacked_borrows/alias_through_mutation.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/alias_through_mutation.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/alias_through_mutation.rs:11
actual stderr:




tests/compile-fail/stacked_borrows/invalidate_against_barrier1.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/invalidate_against_barrier1.rs"
Fail got exit status: 0


`protect` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/invalidate_against_barrier1.rs:4
actual stderr:




tests/compile-fail/stacked_borrows/shared_rw_borrows_are_weak2.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/shared_rw_borrows_are_weak2.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/shared_rw_borrows_are_weak2.rs:13
actual stderr:




tests/compile-fail/stacked_borrows/mut_exclusive_violation2.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/mut_exclusive_violation2.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/mut_exclusive_violation2.rs:8
actual stderr:




tests/compile-fail/stacked_borrows/pass_invalid_mut.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/pass_invalid_mut.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/pass_invalid_mut.rs:8
actual stderr:




tests/compile-fail/stacked_borrows/return_invalid_mut_tuple.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/return_invalid_mut_tuple.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/return_invalid_mut_tuple.rs:10
actual stderr:




tests/compile-fail/stacked_borrows/illegal_read3.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_read3.rs" "-Zmiri-allow-ptr-int-transmute"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_read3.rs:18
actual stderr:




tests/compile-fail/stacked_borrows/illegal_read4.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_read4.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_read4.rs:7
actual stderr:




tests/compile-fail/stacked_borrows/unescaped_static.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/unescaped_static.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/unescaped_static.rs:5
actual stderr:




tests/compile-fail/stacked_borrows/illegal_read1.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_read1.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_read1.rs:9
actual stderr:




tests/compile-fail/stacked_borrows/illegal_read5.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_read5.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_read5.rs:15
actual stderr:




tests/compile-fail/stacked_borrows/pointer_smuggling.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/pointer_smuggling.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/pointer_smuggling.rs:10
actual stderr:




tests/compile-fail/stacked_borrows/illegal_read8.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_read8.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_read8.rs:11
actual stderr:




tests/compile-fail/stacked_borrows/illegal_write5.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/illegal_write5.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/illegal_write5.rs:10
actual stderr:




tests/compile-fail/stacked_borrows/load_invalid_mut.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/load_invalid_mut.rs" "-Zmiri-disable-validation"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/load_invalid_mut.rs:10
actual stderr:




tests/compile-fail/stacked_borrows/return_invalid_mut.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/stacked_borrows/return_invalid_mut.rs"
Fail got exit status: 0


`borrow stack` not found in stderr output
expected because of pattern here: tests/compile-fail/stacked_borrows/return_invalid_mut.rs:5
actual stderr:




tests/compile-fail/concurrency/thread_local_static_dealloc.rs FAILED
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zui-testing" "tests/compile-fail/concurrency/thread_local_static_dealloc.rs"
Fail got exit status: 0


`dereferenced after this allocation got freed` not found in stderr output
expected because of pattern here: tests/compile-fail/concurrency/thread_local_static_dealloc.rs:11
actual stderr:
warning: thread support is experimental and incomplete: weak memory effects are not emulated.
---
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:00
