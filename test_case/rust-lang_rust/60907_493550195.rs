plain
travis_time:end:098e50e3:start=1558110710712528390,finish=1558110796840762463,duration=86128234073
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:34] 
[01:19:34] running 143 tests
[01:19:37] i..iii.....iii..iiii.....i......................i..i.................i......i.........ii.i..i..i.ii. 100/143
[01:19:39] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:39] 
[01:19:39]  finished in 4.653
[01:19:39] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:41] 
[01:19:41] running 9 tests
[01:19:41] iiiiiiiii
[01:19:41] 
[01:19:41]  finished in 0.149
[01:19:41] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:57] 
[01:19:57] running 122 tests
[01:20:21] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:20:26] .i.i......iii.i.....ii
[01:20:26] 
[01:20:26]  finished in 29.869
[01:20:26] travis_fold:end:test_debuginfo

---
[01:44:06] 
[01:44:06] running 26 tests
[01:44:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:44:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:44:19] ............F........F....
[01:44:19] 
[01:44:19] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:44:19] diff of stdout:
[01:44:19] 
[01:44:19] 
[01:44:19] 15 error: aborting due to previous error
[01:44:19] 16 
[01:44:19] 17 For more information about this error, try `rustc --explain E0425`.
[01:44:19] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:44:19] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:44:19] 20 
[01:44:19] 20 
[01:44:19] 21 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:44:19] 
[01:44:19] 24 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:44:19] 26 
[01:44:19] - ', src/librustdoc/test.rs:341:17
[01:44:19] + ', src/librustdoc/test.rs:343:17
[01:44:19] 28 
[01:44:19] 28 
[01:44:19] 29 
[01:44:19] 30 failures:
[01:44:19] 
[01:44:19] 
[01:44:19] The actual stdout differed from the expected stdout.
[01:44:19] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:44:19] To update references, rerun the tests and pass the `--bless` flag
[01:44:19] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:44:19] error: 1 errors occurred comparing output.
[01:44:19] status: exit code: 101
[01:44:19] status: exit code: 101
[01:44:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:44:19] ------------------------------------------
[01:44:19] 
[01:44:19] running 2 tests
[01:44:19] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:44:19] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:44:19] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:44:19] failures:
[01:44:19] 
[01:44:19] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:44:19] error[E0425]: cannot find value `no` in this scope
---
[01:44:19] For more information about this error, try `rustc --explain E0425`.
[01:44:19] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:44:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:44:19] 
[01:44:19] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:44:19] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:44:19] 
[01:44:19] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:44:19] 
[01:44:19] ', src/librustdoc/test.rs:343:17
[01:44:19] 
[01:44:19] 
[01:44:19] 
[01:44:19] failures:
[01:44:19]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:44:19]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:44:19] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:19] 
[01:44:19] 
[01:44:19] ------------------------------------------
---
[01:44:19] 
[01:44:19] 13 
[01:44:19] 14 error: aborting due to previous error
[01:44:19] 15 
[01:44:19] - thread '$DIR/unparseable-doc-test.rs - foo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:44:19] + thread '$DIR/unparseable-doc-test.rs - foo (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:44:19] 18 
[01:44:19] 19 
[01:44:19] 
[01:44:19] 
[01:44:19] 
[01:44:19] The actual stdout differed from the expected stdout.
[01:44:19] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test/unparseable-doc-test.stdout
[01:44:19] To update references, rerun the tests and pass the `--bless` flag
[01:44:19] To only update this specific test, also pass `--test-args unparseable-doc-test.rs`
[01:44:19] error: 1 errors occurred comparing output.
[01:44:19] status: exit code: 101
[01:44:19] status: exit code: 101
[01:44:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unparseable-doc-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test/auxiliary"
[01:44:19] ------------------------------------------
[01:44:19] 
[01:44:19] running 1 test
[01:44:19] test /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - foo (line 6) ... FAILED
---
[01:44:19] test result: FAILED. 24 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:19] 
[01:44:19] 
[01:44:19] 
[01:44:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:44:19] 
[01:44:19] 
[01:44:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:19] Build completed unsuccessfully in 0:36:34
[01:44:19] Build completed unsuccessfully in 0:36:34
[01:44:19] make: *** [check] Error 1
[01:44:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11f4675e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 18:17:45 UTC 2019
---
travis_time:end:00844f72:start=1558117067432131979,finish=1558117067493213399,duration=61081420
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01203c3c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2c87769a
$ dmesg | grep -i kill
