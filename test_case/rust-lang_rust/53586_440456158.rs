plain
travis_time:end:0226192c:start=1542747800297224863,finish=1542747801448232013,duration=1151007150
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:48] .................................................................................................... 100/5042
[00:49:51] .................................................................................................... 200/5042
[00:49:54] .............................ii............................................ii....................ii. 300/5042
[00:49:56] ..............................................................................................iii... 400/5042
[00:49:59] .....iiiiiiii.iii............................iii...........................................i........ 500/5042
[00:50:06] .................................................................................................... 700/5042
[00:50:12] ..................................................................................i...........i..... 800/5042
[00:50:16] .................................................................................................... 900/5042
[00:50:19] .ii.iii.................ii.iiii..................................................................... 1000/5042
---
[00:50:54] .................................................................................................... 2200/5042
[00:50:58] .................................................................................................... 2300/5042
[00:51:02] .................................................................................................... 2400/5042
[00:51:05] .................................................................................................... 2500/5042
[00:51:09] .....................................................................................iiiiiiiii...... 2600/5042
[00:51:15] ...................................................ii............................................... 2800/5042
[00:51:18] .................................................................................................... 2900/5042
[00:51:22] .................................................................................................... 3000/5042
[00:51:25] ...............................................i.................................................... 3100/5042
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:04] 
[01:05:04] running 117 tests
[01:05:07] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii...............i..ii..i 100/117
[01:05:07] i.i.....iiii.....
[01:05:07] 
[01:05:07]  finished in 3.403
[01:05:07] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:22] 
[01:05:22] running 118 tests
[01:05:46] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:50] ......iii.i.....ii
[01:05:50] 
[01:05:50]  finished in 28.499
[01:05:50] travis_fold:end:test_debuginfo

---
[01:41:26] 
[01:41:26] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:41:26] diff of stdout:
[01:41:26] 
[01:41:26] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:41:26] 23 
[01:41:26] - 
[01:41:26] - 
[01:41:26] - thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:41:26] - note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:41:26] 28 ', src/librustdoc/test.rs:358:17
[01:41:26] 29 
[01:41:26] 30 
[01:41:26] 
[01:41:26] 
[01:41:26] 
[01:41:26] The actual stdout differed from the expected stdout.
[01:41:26] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:41:26] To update references, rerun the tests and pass the `--bless` flag
[01:41:26] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:41:26] error: 1 errors occurred comparing output.
[01:41:26] status: exit code: 101
[01:41:26] status: exit code: 101
[01:41:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:41:26] ------------------------------------------
[01:41:26] 
[01:41:26] running 2 tests
[01:41:26] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:41:26] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:41:26] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) ... FAILED
[01:41:26] failures:
[01:41:26] 
[01:41:26] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:41:26] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:41:26] error[E0425]: cannot find value `no` in this scope
[01:41:26]   |
[01:41:26] 3 | no
[01:41:26]   | ^^ not found in this scope
[01:41:26] 
[01:41:26] 
[01:41:26] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:41:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:41:26] 
[01:41:26] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:41:26] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)' panicked at 'test executable failed:
[01:41:26] 
[01:41:26] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:41:26] 
[01:41:26] ', src/librustdoc/test.rs:358:17
[01:41:26] 
[01:41:26] 
[01:41:26] 
[01:41:26] failures:
[01:41:26]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)
[01:41:26]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)
[01:41:26] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:41:26] 
[01:41:26] 
[01:41:26] ------------------------------------------
---
[01:41:26] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:41:26] 
[01:41:26] 
[01:41:26] 
[01:41:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:41:26] 
[01:41:26] 
[01:41:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:26] Build completed unsuccessfully in 0:55:21
[01:41:26] Build completed unsuccessfully in 0:55:21
[01:41:26] Makefile:58: recipe for target 'check' failed
[01:41:26] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16123f44
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 20 22:44:57 UTC 2018
---
travis_time:end:01e9fc09:start=1542753899471195105,finish=1542753899630190031,duration=158994926
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03643554
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0414ef98
$ dmesg | grep -i kill
