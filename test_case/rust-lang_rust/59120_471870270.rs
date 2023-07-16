plain
travis_time:end:02b7ddc3:start=1552366181588906449,finish=1552366183827518846,duration=2238612397
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:17:51] .........................iiiii...................................................................... 1100/5448
[01:17:54] .................................................................................................... 1200/5448
[01:17:57] .................................................................................................... 1300/5448
[01:18:00] .................................................................................................... 1400/5448
[01:18:01] ERROR 2019-03-12T06:07:56Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/auxiliary/pub_and_stability.rs` source not found"
[01:18:03] ........................F........................................................................... 1500/5448
[01:18:09] ..............................i..................................................................... 1700/5448
[01:18:13] .................................................................................................... 1800/5448
[01:18:18] .................................................................................................... 1900/5448
[01:18:21] .................................................................................................... 2000/5448
[01:18:21] .................................................................................................... 2000/5448
[01:18:25] .........................................................i.......................................... 2100/5448
[01:18:27] ERROR 2019-03-12T06:08:22Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/use-from-trait-xc.rs` source not found"
[01:18:29] .......................................F............................................................ 2200/5448
[01:18:38] .................................................................................................... 2400/5448
[01:18:43] .................................................................................................... 2500/5448
[01:18:47] .................................................................................................... 2600/5448
[01:18:51] .................................................................................................... 2700/5448
[01:18:51] .................................................................................................... 2700/5448
[01:18:56] .................................................................................................... 2800/5448
[01:19:00] .................................................................................................... 2900/5448
[01:19:02] ERROR 2019-03-12T06:08:57Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/issue-56411-aux.rs` source not found"
[01:19:05] ..................................F................................................................. 3000/5448
[01:19:08] .................................................................................................... 3100/5448
[01:19:11] ERROR 2019-03-12T06:09:07Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/lint/auxiliary/stability-cfg2.rs` source not found"
[01:19:12] ERROR 2019-03-12T06:09:07Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/lint/auxiliary/stability-cfg2.rs` source not found"
[01:19:13] ........................................................................F......F.................... 3200/5448
[01:19:21] .................................................................................................... 3400/5448
[01:19:25] ................................................ii...i..ii.......................................... 3500/5448
[01:19:29] .................................................................................................... 3600/5448
[01:19:33] .................................................................................................... 3700/5448
[01:19:33] .................................................................................................... 3700/5448
[01:19:37] ..........................................................ii........................................ 3800/5448
[01:19:38] ERROR 2019-03-12T06:09:33Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/auxiliary/orphan_check_diagnostics.rs` source not found"
[01:19:40] .............F..............................................................i....................... 3900/5448
[01:19:44] ..................................i................................................................. 4100/5448
[01:19:48] .................................................................................................... 4200/5448
[01:20:01] .................................................................................................... 4300/5448
[01:20:05] .................................................................................................... 4400/5448
---
[01:20:48] failures:
[01:20:48] 
[01:20:48] ---- [ui] ui/explore-issue-38412.rs stdout ----
[01:20:48] 
[01:20:48] error: aux-build `/checkout/src/test/ui/auxiliary/pub_and_stability.rs` source not found
[01:20:48] thread '[ui] ui/explore-issue-38412.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:20:48] 
[01:20:48] ---- [ui] ui/issues/issue-18986.rs stdout ----
[01:20:48] 
[01:20:48] 
[01:20:48] error: aux-build `/checkout/src/test/ui/issues/auxiliary/use-from-trait-xc.rs` source not found
[01:20:48] thread '[ui] ui/issues/issue-18986.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:20:48] ---- [ui] ui/issues/issue-56411.rs stdout ----
[01:20:48] 
[01:20:48] 
[01:20:48] error: aux-build `/checkout/src/test/ui/issues/auxiliary/issue-56411-aux.rs` source not found
[01:20:48] thread '[ui] ui/issues/issue-56411.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:20:48] ---- [ui] ui/lint/lint-stability-deprecated.rs stdout ----
[01:20:48] 
[01:20:48] 
[01:20:48] error: aux-build `/checkout/src/test/ui/lint/auxiliary/stability-cfg2.rs` source not found
[01:20:48] thread '[ui] ui/lint/lint-stability-deprecated.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:20:48] ---- [ui] ui/lint/lint-stability.rs stdout ----
[01:20:48] 
[01:20:48] 
[01:20:48] error: aux-build `/checkout/src/test/ui/lint/auxiliary/stability-cfg2.rs` source not found
[01:20:48] thread '[ui] ui/lint/lint-stability.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:20:48] ---- [ui] ui/orphan-check-diagnostics.rs stdout ----
[01:20:48] 
[01:20:48] 
[01:20:48] error: aux-build `/checkout/src/test/ui/auxiliary/orphan_check_diagnostics.rs` source not found
[01:20:48] thread '[ui] ui/orphan-check-diagnostics.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:20:48] 
[01:20:48] failures:
[01:20:48]     [ui] ui/explore-issue-38412.rs
[01:20:48]     [ui] ui/issues/issue-18986.rs
---
[01:20:48] 
[01:20:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:20:48] 
[01:20:48] 
[01:20:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:48] 
[01:20:48] 
[01:20:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:48] Build completed unsuccessfully in 0:04:45
[01:20:48] Build completed unsuccessfully in 0:04:45
[01:20:48] Makefile:48: recipe for target 'check' failed
[01:20:48] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c14ac13
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 06:10:43 UTC 2019
