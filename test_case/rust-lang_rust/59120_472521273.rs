plain
travis_time:end:159f7c26:start=1552491765324254624,finish=1552491882611102715,duration=117286848091
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
[01:25:18] 
[01:25:18] running 120 tests
[01:25:46] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:25:51] .i......iii.i.....ii
[01:25:51] 
[01:25:51]  finished in 33.365
[01:25:51] travis_fold:end:test_debuginfo

---
[01:30:05] 
[01:30:05] running 300 tests
[01:31:24] ...........................i........................................................................ 100/300
[01:32:32] ........................................i........................................................... 200/300
[01:32:46] ERROR 2019-03-13T17:17:39Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/rustdoc/auxiliary/enum_primitive.rs` source not found"
[01:33:01] ERROR 2019-03-13T17:17:54Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/rustdoc/auxiliary/reexp_stripped.rs` source not found"
[01:33:40] ......................F.....................F....................................................... 300/300
[01:33:40] failures:
[01:33:40] 
[01:33:40] ---- [rustdoc] rustdoc/no-stack-overflow-25295.rs stdout ----
[01:33:40] 
[01:33:40] 
[01:33:40] error: aux-build `/checkout/src/test/rustdoc/auxiliary/enum_primitive.rs` source not found
[01:33:40] thread '[rustdoc] rustdoc/no-stack-overflow-25295.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2043:9
[01:33:40] ---- [rustdoc] rustdoc/redirect.rs stdout ----
[01:33:40] 
[01:33:40] 
[01:33:40] error: aux-build `/checkout/src/test/rustdoc/auxiliary/reexp_stripped.rs` source not found
[01:33:40] thread '[rustdoc] rustdoc/redirect.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2043:9
[01:33:40] 
[01:33:40] failures:
[01:33:40]     [rustdoc] rustdoc/no-stack-overflow-25295.rs
[01:33:40]     [rustdoc] rustdoc/redirect.rs
[01:33:40]     [rustdoc] rustdoc/redirect.rs
[01:33:40] 
[01:33:40] test result: FAILED. 296 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out
[01:33:40] 
[01:33:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:33:40] 
[01:33:40] 
[01:33:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:33:40] 
[01:33:40] 
[01:33:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:40] Build completed unsuccessfully in 0:21:43
[01:33:40] Build completed unsuccessfully in 0:21:43
[01:33:40] make: *** [check] Error 1
[01:33:40] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1003a70f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 17:18:33 UTC 2019
