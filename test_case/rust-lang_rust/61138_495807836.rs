plain
travis_time:end:001bba93:start=1558733872844623518,finish=1558733873645894496,duration=801270978
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:31] 
[01:08:31] running 5583 tests
[01:08:34] .................................................................................................... 100/5583
[01:08:34] ERROR 2019-05-24T22:46:38Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/async-await/auxiliary/arc_wake.rs` source not found"
[01:08:34] ERROR 2019-05-24T22:46:39Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/async-await/auxiliary/arc_wake.rs` source not found"
[01:08:39] .........F..........F............................................................................... 200/5583
[01:08:45] .................................................................................................... 400/5583
[01:08:49] ...................................................................................................i 500/5583
[01:08:52] .................................................................................................... 600/5583
[01:08:56] .................................................................................................... 700/5583
---
[01:12:00] failures:
[01:12:00] 
[01:12:00] ---- [ui] ui/async-await/async-await.rs stdout ----
[01:12:00] 
[01:12:00] error: aux-build `/checkout/src/test/ui/async-await/auxiliary/arc_wake.rs` source not found
[01:12:00] thread '[ui] ui/async-await/async-await.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2214:9
[01:12:00] 
[01:12:00] ---- [ui] ui/async-await/await-macro.rs stdout ----
[01:12:00] 
[01:12:00] 
[01:12:00] error: aux-build `/checkout/src/test/ui/async-await/auxiliary/arc_wake.rs` source not found
[01:12:00] thread '[ui] ui/async-await/await-macro.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2214:9
[01:12:00] 
[01:12:00] failures:
[01:12:00]     [ui] ui/async-await/async-await.rs
[01:12:00]     [ui] ui/async-await/await-macro.rs
[01:12:00]     [ui] ui/async-await/await-macro.rs
[01:12:00] 
[01:12:00] test result: FAILED. 5560 passed; 2 failed; 21 ignored; 0 measured; 0 filtered out
[01:12:00] 
[01:12:00] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:12:00] 
[01:12:00] 
[01:12:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:00] 
[01:12:00] 
[01:12:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:00] Build completed unsuccessfully in 0:04:45
[01:12:00] Build completed unsuccessfully in 0:04:45
[01:12:00] Makefile:48: recipe for target 'check' failed
[01:12:00] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d89af0a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 22:50:05 UTC 2019
