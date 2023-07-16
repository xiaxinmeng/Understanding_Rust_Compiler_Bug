plain
travis_time:end:1dd45344:start=1544206054859696424,finish=1544206057651895664,duration=2792199240
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:31] 
[00:52:31] running 120 tests
[00:52:34] i..ii...iii...iiii....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:52:34] ..ii.i.....iiii.....
[00:52:34] 
[00:52:34]  finished in 3.385
[00:52:34] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:48] 
[00:52:48] running 118 tests
[00:53:11] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:53:15] ......iii.i.....ii
[00:53:15] 
[00:53:15]  finished in 26.731
[00:53:15] travis_fold:end:test_debuginfo

---
[01:22:30] 
[01:22:30] 12 3 | no
[01:22:30] 13   | ^^ not found in this scope
[01:22:30] 14 
[01:22:30] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:327:13
[01:22:30] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:22:30] 17 
[01:22:30] 17 
[01:22:30] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:22:30] 
[01:22:30] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:22:30] 23 
[01:22:30] - ', src/librustdoc/test.rs:362:17
[01:22:30] + ', src/librustdoc/test.rs:361:17
[01:22:30] 25 
[01:22:30] 25 
[01:22:30] 26 
[01:22:30] 27 failures:
[01:22:30] 
[01:22:30] 
[01:22:30] The actual stdout differed from the expected stdout.
[01:22:30] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:22:30] To update references, rerun the tests and pass the `--bless` flag
[01:22:30] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:22:30] error: 1 errors occurred comparing output.
[01:22:30] status: exit code: 101
[01:22:30] status: exit code: 101
[01:22:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:22:30] ------------------------------------------
[01:22:30] 
[01:22:30] running 2 tests
[01:22:30] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:22:30] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:22:30] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) ... FAILED
[01:22:30] failures:
[01:22:30] 
[01:22:30] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:22:30] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:22:30] error[E0425]: cannot find value `no` in this scope
[01:22:30]   |
[01:22:30] 3 | no
[01:22:30]   | ^^ not found in this scope
[01:22:30] 
[01:22:30] 
[01:22:30] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:22:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:22:30] 
[01:22:30] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:22:30] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)' panicked at 'test executable failed:
[01:22:30] 
[01:22:30] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:22:30] 
[01:22:30] ', src/librustdoc/test.rs:361:17
[01:22:30] 
[01:22:30] 
[01:22:30] 
[01:22:30] failures:
[01:22:30]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)
[01:22:30]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)
[01:22:30] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:22:30] 
[01:22:30] 
[01:22:30] ------------------------------------------
---
[01:22:30] test result: FAILED. 9 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:22:30] 
[01:22:30] 
[01:22:30] 
[01:22:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:30] 
[01:22:30] 
[01:22:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:30] Build completed unsuccessfully in 0:40:16
[01:22:30] Build completed unsuccessfully in 0:40:16
[01:22:30] Makefile:58: recipe for target 'check' failed
[01:22:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:049aefae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 19:30:16 UTC 2018
