plain
travis_time:end:0c558e49:start=1556812637509286900,finish=1556812724938367146,duration=87429080246
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
[01:21:59] 
[01:21:59] running 9 tests
[01:21:59] iiiiiiiii
[01:21:59] 
[01:21:59]  finished in 0.155
[01:21:59] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:15] 
[01:22:15] running 121 tests
[01:22:41] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:22:46] i.i......iii.i.....ii
[01:22:46] 
[01:22:46]  finished in 31.211
[01:22:46] travis_fold:end:test_debuginfo

---
[01:48:44] 
[01:48:44] running 12 tests
[01:48:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:48:45] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:48:45] iiiiiiiii..F
[01:48:45] 
[01:48:45] ---- [run-make] run-make/rustc-macro-dep-files stdout ----
[01:48:45] 
[01:48:45] error: make failed
[01:48:45] error: make failed
[01:48:45] status: exit code: 2
[01:48:45] command: "make"
[01:48:45] stdout:
[01:48:45] ------------------------------------------
[01:48:45] make[1]: Entering directory '/checkout/src/test/run-make/rustc-macro-dep-files'
[01:48:45] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rustc-macro-dep-files/rustc-macro-dep-files:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc'  -Clinker= foo.rs --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rustc-macro-dep-files/rustc-macro-dep-files
[01:48:45] Makefile:6: recipe for target 'all' failed
[01:48:45] 
[01:48:45] ------------------------------------------
[01:48:45] stderr:
[01:48:45] ------------------------------------------
[01:48:45] ------------------------------------------
[01:48:45] error: couldn't extract file stem from specified linker
[01:48:45] error: aborting due to previous error
[01:48:45] 
[01:48:45] 
[01:48:45] make[1]: *** [all] Error 1
[01:48:45] ------------------------------------------
[01:48:45] 
[01:48:45] 
[01:48:45] 
[01:48:45] 
[01:48:45] failures:
[01:48:45]     [run-make] run-make/rustc-macro-dep-files
[01:48:45] 
[01:48:45] test result: FAILED. 2 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
[01:48:45] 
[01:48:45] 
[01:48:45] 
[01:48:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:48:45] 
[01:48:45] 
[01:48:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:45] Build completed unsuccessfully in 0:38:46
[01:48:45] Build completed unsuccessfully in 0:38:46
[01:48:45] make: *** [check] Error 1
[01:48:45] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0994f59e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 17:47:40 UTC 2019
---
travis_time:end:019c9432:start=1556819262371956843,finish=1556819262444906035,duration=72949192
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c21fdb8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:151cfb7b
$ dmesg | grep -i kill
