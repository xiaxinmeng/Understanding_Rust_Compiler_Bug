plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 68314177e70017c08f6cdf295631bb508f9f85bc and 207c7e49a37c7c66e432851d51394bf9fbd90c26
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
error: tests/compile-fail/alloc/reallocate-change-alloc.rs:7: expected error not found: dereferenced after this allocation got freed

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/alloc/reallocate-change-alloc.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/alloc/reallocate-change-alloc.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/alloc/reallocate-change-alloc.stage-id.aux"
   1: <compiletest_rs::runtest::TestCx>::run_cfail_test
   2: compiletest_rs::runtest::run
   3: <compiletest_rs::make_test_closure::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
not found errors (from test file): [
---
test [compile-fail] compile-fail/dangling_pointers/deref-invalid-ptr.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/concurrency/thread_local_static_dealloc.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/concurrency/thread_local_static_dealloc.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/concurrency/thread_local_static_dealloc.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/data_race/dangling_thread_async_race.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/environ-gets-deallocated.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/environ-gets-deallocated.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/environ-gets-deallocated.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/data_race/rmw_race.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/extern_static.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/extern_static.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/extern_static.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/intrinsics/uninit_uninhabited_type.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/invalid_bool.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/invalid_bool.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "-L" "/tmp/compiletestATZEju/invalid_bool.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/no_main.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/invalid_char.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/invalid_char.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-alignment-check" "-Zmiri-disable-stacked-borrows" "-Zmiri-disable-validation" "-L" "/tmp/compiletestATZEju/invalid_char.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/rustc-error.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/pointer_partial_read.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/pointer_partial_read.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/pointer_partial_read.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/panic/panic_abort2.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/reading_half_a_pointer.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/reading_half_a_pointer.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/reading_half_a_pointer.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/aliasing_mut3.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/alias_through_mutation.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/alias_through_mutation.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/alias_through_mutation.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/deallocate_against_barrier2.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_read3.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_read3.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_read3.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/illegal_read3.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_read2.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_read2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_read2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/illegal_read2.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_read1.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_read1.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_read1.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/illegal_read1.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_read4.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_read4.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_read4.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

error: compile-fail test compiled successfully!
thread '[compile-fail] compile-fail/stacked_borrows/illegal_write1.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/runtest.rs:1092:13
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_read6.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_read6.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_read6.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_read5.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_read5.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_read5.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
error: tests/compile-fail/stacked_borrows/illegal_write1.rs:8: expected error not found: borrow stack

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_write1.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_write1.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_write1.stage-id.aux"
    Error {
        line_num: 8,
        kind: Some(
            Error,
---
test [compile-fail] compile-fail/stacked_borrows/illegal_write2.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_read8.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_read8.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_read8.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/illegal_read8.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_write4.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_write4.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_write4.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/interior_mut1.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/illegal_write5.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/illegal_write5.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/illegal_write5.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/invalidate_against_barrier2.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/invalidate_against_barrier1.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/invalidate_against_barrier1.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/invalidate_against_barrier1.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/load_invalid_shr.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/load_invalid_mut.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/load_invalid_mut.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-validation" "-L" "/tmp/compiletestATZEju/stacked_borrows/load_invalid_mut.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/outdated_local.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/mut_exclusive_violation2.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/mut_exclusive_violation2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/mut_exclusive_violation2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/mut_exclusive_violation2.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/pointer_smuggling.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/pointer_smuggling.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/pointer_smuggling.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/raw_tracking.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/return_invalid_mut.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/return_invalid_mut.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/return_invalid_mut.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/return_invalid_mut.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/pass_invalid_mut.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/pass_invalid_mut.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/pass_invalid_mut.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/pass_invalid_mut.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/return_invalid_mut_option.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/return_invalid_mut_option.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/return_invalid_mut_option.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/return_invalid_shr.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/return_invalid_mut_tuple.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/return_invalid_mut_tuple.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/return_invalid_mut_tuple.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/shr_frozen_violation1.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/shared_rw_borrows_are_weak2.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/shared_rw_borrows_are_weak2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/shared_rw_borrows_are_weak2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/stacked_borrows/transmute-is-no-escape.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/unescaped_static.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/stacked_borrows/unescaped_static.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/stacked_borrows/unescaped_static.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/type-too-large.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/type-too-large.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/type-too-large.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/sync/libc_pthread_rwlock_write_write_deadlock.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unaligned_pointers/reference_to_packed.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/unaligned_pointers/reference_to_packed.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows" "-L" "/tmp/compiletestATZEju/unaligned_pointers/reference_to_packed.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/unaligned_pointers/reference_to_packed.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unaligned_pointers/unaligned_ptr1.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr1.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows" "-L" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr1.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/unaligned_pointers/unaligned_ptr1.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unaligned_pointers/unaligned_ptr3.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr3.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows" "-L" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr3.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unaligned_pointers/unaligned_ptr4.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr4.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows" "-L" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr4.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/unaligned_pointers/unaligned_ptr4.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unaligned_pointers/unaligned_ptr2.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows" "-L" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/unaligned_pointers/unaligned_ptr2.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unaligned_pointers/unaligned_ptr_addr_of.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr_addr_of.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows" "-L" "/tmp/compiletestATZEju/unaligned_pointers/unaligned_ptr_addr_of.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/validity/invalid_char.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_bool_uninit.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/validity/invalid_bool_uninit.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/validity/invalid_bool_uninit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/validity/nonzero.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_char_uninit.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/validity/invalid_char_uninit.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/validity/invalid_char_uninit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/validity/invalid_char_uninit.rs ... FAILED

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_enum_tag_256variants_uninit.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/validity/invalid_enum_tag_256variants_uninit.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/validity/invalid_enum_tag_256variants_uninit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/validity/invalid_wide_raw.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_fnptr_uninit.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/validity/invalid_fnptr_uninit.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/validity/invalid_fnptr_uninit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/zst2.rs ... ok

error: compile-fail test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/zst1.rs" "-L" "/tmp/compiletestATZEju" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestATZEju/zst1.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-O" "-Zmir-opt-level=4" "-L" "/tmp/compiletestATZEju/zst1.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
......... (60/64)
....       (64/64)


/checkout/src/test/rustdoc-gui/settings.goml settings... FAILED
[ERROR] (line 70) Error: Evaluation failed: expected `null` for key `display` for selector `noscript`, found `inline`: for command `assert-css: ("noscript", {"display": "null"})`
Build completed unsuccessfully in 0:00:20
