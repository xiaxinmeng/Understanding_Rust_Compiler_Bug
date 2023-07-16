plain
travis_time:end:1a0c345c:start=1551027363873184199,finish=1551027365111917544,duration=1238733345
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
[01:12:51] 
[01:12:51] running 119 tests
[01:13:16] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:21] i......iii.i.....ii
[01:13:21] 
[01:13:21]  finished in 30.085
[01:13:21] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:42:31] 
[01:42:31] running 15 tests
[01:42:31] ..ii.iiiiiii...
[01:42:31] 
[01:42:31]  finished in 0.645
[01:42:31] travis_fold:end:test_run-make

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:42:32] 
[01:42:32] running 14 tests
[01:42:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:42:37] ...F..........
[01:42:37] 
[01:42:37] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:42:37] diff of stdout:
[01:42:37] 
[01:42:37] 
[01:42:37] 12 3 | no
[01:42:37] 13   | ^^ not found in this scope
[01:42:37] 14 
[01:42:37] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:42:37] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:42:37] 17 
[01:42:37] 17 
[01:42:37] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:42:37] 
[01:42:37] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:42:37] 23 
[01:42:37] - ', src/librustdoc/test.rs:372:17
[01:42:37] + ', src/librustdoc/test.rs:378:17
[01:42:37] 25 
[01:42:37] 25 
[01:42:37] 26 
[01:42:37] 27 failures:
[01:42:37] 
[01:42:37] 
[01:42:37] The actual stdout differed from the expected stdout.
[01:42:37] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:42:37] To update references, rerun the tests and pass the `--bless` flag
[01:42:37] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:42:37] error: 1 errors occurred comparing output.
[01:42:37] status: exit code: 101
[01:42:37] status: exit code: 101
[01:42:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:42:37] ------------------------------------------
[01:42:37] 
[01:42:37] running 2 tests
[01:42:37] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:42:37] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:42:37] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:42:37] failures:
[01:42:37] 
[01:42:37] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:42:37] error[E0425]: cannot find value `no` in this scope
---
[01:42:37] 
[01:42:37] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:42:37] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:37] 
[01:42:37] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:42:37] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:42:37] 
[01:42:37] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:42:37] 
[01:42:37] ', src/librustdoc/test.rs:378:17
[01:42:37] 
[01:42:37] 
[01:42:37] 
[01:42:37] failures:
[01:42:37]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:42:37]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:42:37] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:37] 
[01:42:37] 
[01:42:37] ------------------------------------------
---
[01:42:37] test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:37] 
[01:42:37] 
[01:42:37] 
[01:42:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:37] 
[01:42:37] 
[01:42:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:37] Build completed unsuccessfully in 0:41:31
[01:42:37] Build completed unsuccessfully in 0:41:31
[01:42:37] Makefile:48: recipe for target 'check' failed
[01:42:37] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07b13ec6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 24 18:38:53 UTC 2019
---
travis_time:end:0daf9ddc:start=1551033535443243989,finish=1551033535498220703,duration=54976714
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b1b5a25
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e0b0bcd
$ dmesg | grep -i kill
