plain
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: 1 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/never_transmute_void.rs" "-L" "/tmp/compiletestMzOd5m" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMzOd5m/never_transmute_void.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletestMzOd5m/never_transmute_void.stage-id.aux"
    Error {
        line_num: 15,
        kind: Some(
            Error,
---
This PR updated 'src/tools/miri', verifying if status is 'test-pass'...

We detected that this PR updated 'miri', but its tests failed.

If you do intend to update 'miri', please check the error messages above and
commit another update.

If you do NOT intend to update 'miri', please ensure you did not accidentally
change the submodule at 'src/tools/miri'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:00
