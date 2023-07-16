plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
error: tests/compile-fail/backtrace/bad-backtrace-ptr.rs:7: expected error not found: 0x0 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/backtrace/bad-backtrace-ptr.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/backtrace/bad-backtrace-ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/backtrace/bad-backtrace-ptr.stage-id.aux"
   1: compiletest_rs::runtest::TestCx::run_cfail_test
   2: compiletest_rs::runtest::run
   3: core::ops::function::FnOnce::call_once{{vtable.shim}}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
---
error: tests/compile-fail/dangling_pointers/deref-invalid-ptr.rs:6: expected error not found: inbounds test failed: 0x10 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/dangling_pointers/deref-invalid-ptr.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/dangling_pointers/deref-invalid-ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletestdsvFae/dangling_pointers/deref-invalid-ptr.stage-id.aux"
    Error {
        line_num: 6,
        kind: Some(
            Error,
---
error: tests/compile-fail/dangling_pointers/wild_pointer_deref.rs:3: expected error not found: inbounds test failed: 0x2c is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/dangling_pointers/wild_pointer_deref.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/dangling_pointers/wild_pointer_deref.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/dangling_pointers/wild_pointer_deref.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---
error: tests/compile-fail/function_pointers/cast_int_to_fn_ptr.rs:9: expected error not found: inbounds test failed: 0x2a is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/function_pointers/cast_int_to_fn_ptr.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/function_pointers/cast_int_to_fn_ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletestdsvFae/function_pointers/cast_int_to_fn_ptr.stage-id.aux"
    Error {
        line_num: 9,
        kind: Some(
            Error,
---
test [compile-fail] compile-fail/intrinsics/out_of_bounds_ptr_2.rs ... ok

error: error pattern ' inbounds test failed: 0x0 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/ptr_offset_0_plus_0.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/intrinsics/ptr_offset_0_plus_0.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/intrinsics/ptr_offset_0_plus_0.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/intrinsics/ptr_offset_0_plus_0.rs ... FAILED

error: error pattern ' inbounds test failed: 0x1 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/ptr_offset_int_plus_int.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/intrinsics/ptr_offset_int_plus_int.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/intrinsics/ptr_offset_int_plus_int.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [compile-fail] compile-fail/intrinsics/ptr_offset_int_plus_int.rs ... FAILED

error: error pattern ' inbounds test failed: 0x1 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/ptr_offset_int_plus_ptr.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/intrinsics/ptr_offset_int_plus_ptr.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/intrinsics/ptr_offset_int_plus_ptr.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
error: tests/compile-fail/null_pointer_deref.rs:3: expected error not found: inbounds test failed: 0x0 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_deref.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/null_pointer_deref.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/null_pointer_deref.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---
error: tests/compile-fail/null_pointer_write.rs:3: expected error not found: inbounds test failed: 0x0 is not a valid pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/null_pointer_write.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/null_pointer_write.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/null_pointer_write.stage-id.aux"
    Error {
        line_num: 3,
        kind: Some(
            Error,
---

test [compile-fail] compile-fail/null_pointer_write_zst.rs ... ok
test [compile-fail] compile-fail/null_pointer_write.rs ... FAILED
test [compile-fail] compile-fail/panic/bad_unwind1.rs ... ok
test [compile-fail] compile-fail/non_extern_foreign_call_bad_unwind1.rs ... ok
test [compile-fail] compile-fail/non_extern_foreign_call_abi_mismatch.rs ... ok
test [compile-fail] compile-fail/panic/bad_unwind2.rs ... ok
test [compile-fail] compile-fail/panic/unwind_panic_abort.rs ... ok
test [compile-fail] compile-fail/pointer_byte_read.rs ... ok
test [compile-fail] compile-fail/panic/bad_unwind3.rs ... ok
---
test [compile-fail] compile-fail/stacked_borrows/load_invalid_shr.rs ... ok

error: error pattern ' inbounds test failed: 0x4 is not a valid pointer' not found!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stacked_borrows/issue-miri-1050-2.rs" "-L" "/tmp/compiletestdsvFae" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdsvFae/stacked_borrows/issue-miri-1050-2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdsvFae/stacked_borrows/issue-miri-1050-2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
Verifying status of rustfmt...
Verifying status of miri...
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
proper steps.
{"book":"test-pass","rust-by-example":"test-pass","embedded-book":"test-pass","rustbook":"test-fail","edition-guide":"test-pass","reference":"test-pass","rls":"test-pass","nomicon":"test-pass","miri":"test-fail","rustfmt":"test-pass","cargo-miri":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
