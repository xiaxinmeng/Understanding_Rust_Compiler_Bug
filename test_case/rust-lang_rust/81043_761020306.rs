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
test [ui] run-pass/concurrency/disable_data_race_detector.rs ... ok
test [ui] run-pass/concurrency/issue1643.rs ... ok
test [ui] run-pass/concurrency/concurrent_caller_location.rs ... ok
normalized stderr:
warning: use of deprecated function `std::sync::atomic::spin_loop_hint`: use hint::spin_loop instead
   |
   |
53 |     atomic::spin_loop_hint();
   |
   = note: `#[warn(deprecated)]` on by default




expected stderr:


diff of stderr:

+warning: use of deprecated function `std::sync::atomic::spin_loop_hint`: use hint::spin_loop instead
+   |
+   |
+53 |     atomic::spin_loop_hint();
+   |
+   = note: `#[warn(deprecated)]` on by default
+
+
+

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletesttcvzkx/concurrency/sync_singlethread.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletesttcvzkx' 'concurrency/sync_singlethread.rs'
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/concurrency/sync_singlethread.rs" "-L" "/tmp/compiletesttcvzkx" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesttcvzkx/concurrency/sync_singlethread.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesttcvzkx/concurrency/sync_singlethread.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of deprecated function `std::sync::atomic::spin_loop_hint`: use hint::spin_loop instead","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/concurrency/sync_singlethread.rs","byte_start":1171,"byte_end":1193,"line_start":53,"line_end":53,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    atomic::spin_loop_hint();","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[warn(deprecated)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: use of deprecated function `std::sync::atomic::spin_loop_hint`: use hint::spin_loop instead\n  --> tests/run-pass/concurrency/sync_singlethread.rs:53:5\n   |\n53 |     atomic::spin_loop_hint();\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `#[warn(deprecated)]` on by default\n\n"}
------------------------------------------

test [ui] run-pass/concurrency/sync_singlethread.rs ... FAILED
test [ui] run-pass/const-vec-of-fns.rs ... ok
---

failures:

failures:
   0: std::panicking::begin_panic
   1: compiletest_rs::run_tests
   2: compiletest::run_tests
   3: compiletest::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

test result: FAILED. 203 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--test compiletest'
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
{"miri":"test-fail","reference":"test-pass","book":"test-pass","cargo-miri":"test-fail","rust-by-example":"test-pass","rustbook":"test-fail","edition-guide":"test-pass","rustfmt":"test-pass","embedded-book":"test-pass","nomicon":"test-pass","rls":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
