plain
travis_time:end:15724698:start=1549511838290054567,finish=1549511840439953983,duration=2149899416
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:29] 
[01:10:29] running 119 tests
[01:10:54] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:59] i......iii.i.....ii
[01:10:59] 
[01:10:59]  finished in 29.288
[01:10:59] travis_fold:end:test_debuginfo

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:38:29] 
[01:38:29] running 14 tests
[01:38:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:38:34] ...F..........
[01:38:34] 
[01:38:34] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:38:34] diff of stdout:
[01:38:34] 
[01:38:34] 
[01:38:34] 12 3 | no
[01:38:34] 13   | ^^ not found in this scope
[01:38:34] 14 
[01:38:34] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:38:34] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:355:13
[01:38:34] 17 
[01:38:34] 17 
[01:38:34] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:38:34] 
[01:38:34] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:38:34] 23 
[01:38:34] - ', src/librustdoc/test.rs:389:17
[01:38:34] + ', src/librustdoc/test.rs:390:17
[01:38:34] 25 
[01:38:34] 25 
[01:38:34] 26 
[01:38:34] 27 failures:
[01:38:34] 
[01:38:34] 
[01:38:34] The actual stdout differed from the expected stdout.
[01:38:34] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:38:34] To update references, rerun the tests and pass the `--bless` flag
[01:38:34] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:38:34] error: 1 errors occurred comparing output.
[01:38:34] status: exit code: 101
[01:38:34] status: exit code: 101
[01:38:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:38:34] ------------------------------------------
[01:38:34] 
[01:38:34] running 2 tests
[01:38:34] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:38:34] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:38:34] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:38:34] failures:
[01:38:34] 
[01:38:34] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:38:34] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:38:34] error[E0425]: cannot find value `no` in this scope
[01:38:34]   |
[01:38:34] 3 | no
[01:38:34]   | ^^ not found in this scope
[01:38:34] 
[01:38:34] 
[01:38:34] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:355:13
[01:38:34] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:38:34] 
[01:38:34] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:38:34] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:38:34] 
[01:38:34] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:38:34] 
[01:38:34] ', src/librustdoc/test.rs:390:17
[01:38:34] 
[01:38:34] 
[01:38:34] 
[01:38:34] failures:
[01:38:34]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:38:34]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:38:34] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:38:34] 
[01:38:34] 
[01:38:34] ------------------------------------------
---
[01:38:34] test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:38:34] 
[01:38:34] 
[01:38:34] 
[01:38:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:38:34] 
[01:38:34] 
[01:38:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:34] Build completed unsuccessfully in 0:39:34
[01:38:34] Build completed unsuccessfully in 0:39:34
[01:38:34] make: *** [check] Error 1
[01:38:34] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15eb4d25
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 05:36:05 UTC 2019
---
travis_time:end:158c8358:start=1549517766971070902,finish=1549517767025900131,duration=54829229
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2479b3e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00fae358
$ dmesg | grep -i kill
