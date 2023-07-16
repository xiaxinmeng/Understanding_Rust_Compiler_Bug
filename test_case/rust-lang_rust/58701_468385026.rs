plain
travis_time:end:0f06b004:start=1551372389388588519,finish=1551372500824873444,duration=111436284925
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
[01:14:25] 
[01:14:25] running 119 tests
[01:14:50] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:54] i......iii.i.....ii
[01:14:54] 
[01:14:54]  finished in 29.239
[01:14:54] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:44:01] 
[01:44:01] running 15 tests
[01:44:01] ..iiiiiii.ii...
[01:44:01] 
[01:44:01]  finished in 0.632
[01:44:01] travis_fold:end:test_run-make

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:44:01] 
[01:44:01] running 16 tests
[01:44:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:44:08] ...F............
[01:44:08] 
[01:44:08] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:44:08] diff of stdout:
[01:44:08] 
[01:44:08] 
[01:44:08] 12 3 | no
[01:44:08] 13   | ^^ not found in this scope
[01:44:08] 14 
[01:44:08] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:44:08] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:44:08] 17 
[01:44:08] 17 
[01:44:08] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:44:08] 
[01:44:08] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:44:08] 23 
[01:44:08] - ', src/librustdoc/test.rs:372:17
[01:44:08] + ', src/librustdoc/test.rs:378:17
[01:44:08] 25 
[01:44:08] 25 
[01:44:08] 26 
[01:44:08] 27 failures:
[01:44:08] 
[01:44:08] 
[01:44:08] The actual stdout differed from the expected stdout.
[01:44:08] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:44:08] To update references, rerun the tests and pass the `--bless` flag
[01:44:08] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:44:08] error: 1 errors occurred comparing output.
[01:44:08] status: exit code: 101
[01:44:08] status: exit code: 101
[01:44:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:44:08] ------------------------------------------
[01:44:08] 
[01:44:08] running 2 tests
[01:44:08] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:44:08] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:44:08] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:44:08] failures:
[01:44:08] 
[01:44:08] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:44:08] error[E0425]: cannot find value `no` in this scope
---
[01:44:08] 
[01:44:08] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:44:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:44:08] 
[01:44:08] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:44:08] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:44:08] 
[01:44:08] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:44:08] 
[01:44:08] ', src/librustdoc/test.rs:378:17
[01:44:08] 
[01:44:08] 
[01:44:08] 
[01:44:08] failures:
[01:44:08]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:44:08]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:44:08] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:08] 
[01:44:08] 
[01:44:08] ------------------------------------------
---
[01:44:08] test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:08] 
[01:44:08] 
[01:44:08] 
[01:44:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:44:08] 
[01:44:08] 
[01:44:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:08] Build completed unsuccessfully in 0:41:35
[01:44:08] Build completed unsuccessfully in 0:41:35
[01:44:08] Makefile:48: recipe for target 'check' failed
[01:44:08] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2974d37c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 28 18:32:38 UTC 2019
---
travis_time:end:0f06552c:start=1551378760133140360,finish=1551378760188589678,duration=55449318
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c587ad0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0eab1de6
$ dmesg | grep -i kill
