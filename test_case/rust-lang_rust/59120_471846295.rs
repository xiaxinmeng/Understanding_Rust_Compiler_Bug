plain
travis_time:end:05278e78:start=1552358360530461908,finish=1552358361450247703,duration=919785795
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:08:30] .................................................................................................... 1800/5448
[01:08:33] .................................................................................................... 1900/5448
[01:08:37] .................................................................................................... 2000/5448
[01:08:40] .........................................................i.......................................... 2100/5448
[01:08:41] ERROR 2019-03-12T03:48:13Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/use-from-trait-xc.rs` source not found"
[01:08:44] .......................................F............................................................ 2200/5448
[01:08:51] .................................................................................................... 2400/5448
[01:08:56] .................................................................................................... 2500/5448
[01:08:59] .................................................................................................... 2600/5448
[01:09:03] .................................................................................................... 2700/5448
[01:09:03] .................................................................................................... 2700/5448
[01:09:07] .................................................................................................... 2800/5448
[01:09:11] .................................................................................................... 2900/5448
[01:09:13] ERROR 2019-03-12T03:48:44Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/issue-56411-aux.rs` source not found"
[01:09:15] ..................................F................................................................. 3000/5448
[01:09:22] .................................................................................................... 3200/5448
[01:09:26] ..........................................................................i......................... 3300/5448
[01:09:29] .................................................................................................... 3400/5448
[01:09:33] ................................................ii...i..ii.......................................... 3500/5448
---
[01:10:46] failures:
[01:10:46] 
[01:10:46] ---- [ui] ui/issues/issue-18986.rs stdout ----
[01:10:46] 
[01:10:46] error: aux-build `/checkout/src/test/ui/issues/auxiliary/use-from-trait-xc.rs` source not found
[01:10:46] thread '[ui] ui/issues/issue-18986.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:10:46] 
[01:10:46] ---- [ui] ui/issues/issue-56411.rs stdout ----
[01:10:46] 
[01:10:46] 
[01:10:46] error: aux-build `/checkout/src/test/ui/issues/auxiliary/issue-56411-aux.rs` source not found
[01:10:46] thread '[ui] ui/issues/issue-56411.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:10:46] 
[01:10:46] failures:
[01:10:46]     [ui] ui/issues/issue-18986.rs
[01:10:46]     [ui] ui/issues/issue-56411.rs
[01:10:46]     [ui] ui/issues/issue-56411.rs
[01:10:46] 
[01:10:46] test result: FAILED. 5424 passed; 2 failed; 22 ignored; 0 measured; 0 filtered out
[01:10:46] 
[01:10:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:10:46] 
[01:10:46] 
[01:10:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:46] 
[01:10:46] 
[01:10:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:46] Build completed unsuccessfully in 0:04:13
[01:10:46] Build completed unsuccessfully in 0:04:13
[01:10:46] Makefile:48: recipe for target 'check' failed
[01:10:46] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b0ca550
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 03:50:18 UTC 2019
---
travis_time:end:08ab29d7:start=1552362619595006212,finish=1552362619602356295,duration=7350083
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05aac2d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e9bad94
$ dmesg | grep -i kill
