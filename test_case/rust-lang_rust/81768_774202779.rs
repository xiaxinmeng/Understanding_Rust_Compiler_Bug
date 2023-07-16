plain

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestbkI1t7/panic/catch_panic.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestbkI1t7' 'panic/catch_panic.rs'
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestbkI1t7" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestbkI1t7/panic/catch_panic.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-symbolic-alignment-check" "-L" "/tmp/compiletestbkI1t7/panic/catch_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
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
{"edition-guide":"test-pass","rls":"build-fail","nomicon":"test-pass","rustbook":"test-fail","cargo-miri":"test-fail","embedded-book":"test-pass","reference":"test-pass","rust-by-example":"test-pass","book":"test-pass","rustfmt":"build-fail","miri":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
