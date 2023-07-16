plain
travis_time:end:0d6903a4:start=1555367122540220872,finish=1555367213255050419,duration=90714829547
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:44] 
[01:14:44] running 9 tests
[01:14:44] iiiiiiiii
[01:14:44] 
[01:14:44]  finished in 0.161
[01:14:44] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:01] 
[01:15:01] running 121 tests
[01:15:30] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:35] i.i......iii.i.....ii
[01:15:35] 
[01:15:35]  finished in 34.288
[01:15:35] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:25] 
[01:19:25] running 305 tests
[01:20:40] ............................i......................F................................................ 100/305
[01:22:50] .................................................................................................... 300/305
[01:22:54] .....
[01:22:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:22:54] failures:
[01:22:54] failures:
[01:22:54] 
[01:22:54] ---- [rustdoc] rustdoc/favicon-path.rs stdout ----
[01:22:54] 
[01:22:54] error: rustdoc failed!
[01:22:54] status: exit code: 1
[01:22:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/favicon-path/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/favicon-path" "/checkout/src/test/rustdoc/favicon-path.rs" "-Z" "unstable-options" "--favicon-path" "./src/librustdoc/html/static/favicon.ico"
[01:22:54] ------------------------------------------
[01:22:54] 
[01:22:54] ------------------------------------------
[01:22:54] stderr:
[01:22:54] stderr:
[01:22:54] ------------------------------------------
[01:22:54] error: option `--favicon-path` argument must be a file
[01:22:54] 
[01:22:54] ------------------------------------------
[01:22:54] 
[01:22:54] thread '[rustdoc] rustdoc/favicon-path.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3469:9
---
[01:22:54] test result: FAILED. 302 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:22:54] 
[01:22:54] 
[01:22:54] 
[01:22:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:54] 
[01:22:54] 
[01:22:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:54] Build completed unsuccessfully in 0:20:43
[01:22:54] Build completed unsuccessfully in 0:20:43
[01:22:54] Makefile:48: recipe for target 'check' failed
[01:22:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25b83298
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 15 23:49:58 UTC 2019
