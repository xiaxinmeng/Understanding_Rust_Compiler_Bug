plain
travis_time:end:069fddd6:start=1552456542590971839,finish=1552456617936718167,duration=75345746328
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:56] 
[01:20:56] running 120 tests
[01:21:25] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:31] .i......iii.i.....ii
[01:21:31] 
[01:21:31]  finished in 34.556
[01:21:31] travis_fold:end:test_debuginfo

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:31] 
[01:21:31] running 20 tests
[01:21:31] ERROR 2019-03-13T07:18:38Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui-fulldeps/issues/auxiliary/lint_for_crate.rs` source not found"
[01:21:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:21:37] ..F.................
[01:21:37] 
[01:21:37] ---- [ui] ui-fulldeps/issues/issue-15778-fail.rs stdout ----
[01:21:37] 
[01:21:37] 
[01:21:37] error: aux-build `/checkout/src/test/ui-fulldeps/issues/auxiliary/lint_for_crate.rs` source not found
[01:21:37] thread '[ui] ui-fulldeps/issues/issue-15778-fail.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2043:9
[01:21:37] 
[01:21:37] 
[01:21:37] failures:
[01:21:37]     [ui] ui-fulldeps/issues/issue-15778-fail.rs
[01:21:37]     [ui] ui-fulldeps/issues/issue-15778-fail.rs
[01:21:37] 
[01:21:37] test result: FAILED. 19 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:21:37] 
[01:21:37] 
[01:21:37] 
[01:21:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:37] 
[01:21:37] 
[01:21:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:37] Build completed unsuccessfully in 0:13:35
[01:21:37] Build completed unsuccessfully in 0:13:35
[01:21:37] Makefile:48: recipe for target 'check' failed
[01:21:37] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2dc4c14c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 07:18:45 UTC 2019
---
travis_time:end:0bea7e57:start=1552461527777226211,finish=1552461527782457726,duration=5231515
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:058c764c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj
