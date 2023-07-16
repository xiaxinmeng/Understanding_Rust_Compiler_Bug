plain
travis_time:end:01f5bc37:start=1543018741490622165,finish=1543018798753373196,duration=57262751031
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:20] 
[00:50:20] running 5050 tests
[00:50:23] .................................................................................................... 100/5050
[00:50:26] .................................................................................................... 200/5050
[00:50:29] .............................ii............................................ii...................ii.. 300/5050
[00:50:32] ...............................................................................................iii.. 400/5050
[00:50:35] .....iiiiiiii.iii............................iii...........................................i........ 500/5050
[00:50:42] .................................................................................................... 700/5050
[00:50:48] ...................................................................................i...........i.... 800/5050
[00:50:52] .................................................................................................... 900/5050
[00:50:55] ..iiiii..................ii.iiii.................................................................... 1000/5050
---
[00:51:32] .................................................................................................... 2200/5050
[00:51:36] .................................................................................................... 2300/5050
[00:51:40] .................................................................................................... 2400/5050
[00:51:44] .................................................................................................... 2500/5050
[00:51:47] ...........................................................................................iiiiiiiii 2600/5050
[00:51:54] .........................................................ii......................................... 2800/5050
[00:51:57] .................................................................................................... 2900/5050
[00:52:01] .................................................................................................... 3000/5050
[00:52:05] ......................................................i............................................. 3100/5050
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:16] 
[01:06:16] running 117 tests
[01:06:19] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:06:20] i.i.....iiii.....
[01:06:20] 
[01:06:20]  finished in 3.731
[01:06:20] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:35] 
[01:06:35] running 118 tests
[01:06:59] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:07:03] ......iii.i.....ii
[01:07:03] 
[01:07:03]  finished in 27.737
[01:07:03] travis_fold:end:test_debuginfo

---
[01:42:10] 
[01:42:10] 12 3 | no
[01:42:10] 13   | ^^ not found in this scope
[01:42:10] 14 
[01:42:10] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:42:10] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:368:13
[01:42:10] 17 
[01:42:10] 17 
[01:42:10] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:42:10] 
[01:42:10] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:42:10] 23 
[01:42:10] - ', src/librustdoc/test.rs:358:17
[01:42:10] + ', src/librustdoc/test.rs:414:17
[01:42:10] 25 
[01:42:10] 25 
[01:42:10] 26 
[01:42:10] 27 failures:
[01:42:10] 
[01:42:10] 
[01:42:10] The actual stdout differed from the expected stdout.
[01:42:10] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:42:10] To update references, rerun the tests and pass the `--bless` flag
[01:42:10] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:42:10] error: 1 errors occurred comparing output.
[01:42:10] status: exit code: 101
[01:42:10] status: exit code: 101
[01:42:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:42:10] ------------------------------------------
[01:42:10] 
[01:42:10] running 2 tests
[01:42:10] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:42:10] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:42:10] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) ... FAILED
[01:42:10] failures:
[01:42:10] 
[01:42:10] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:42:10] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:42:10] error[E0425]: cannot find value `no` in this scope
[01:42:10]   |
[01:42:10] 3 | no
[01:42:10]   | ^^ not found in this scope
[01:42:10] 
[01:42:10] 
[01:42:10] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:368:13
[01:42:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:42:10] 
[01:42:10] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:42:10] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)' panicked at 'test executable failed:
[01:42:10] 
[01:42:10] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:42:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:42:10] 
[01:42:10] ', src/librustdoc/test.rs:414:17
[01:42:10] 
[01:42:10] 
[01:42:10] 
[01:42:10] failures:
[01:42:10]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)
[01:42:10]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)
[01:42:10] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:10] 
[01:42:10] 
[01:42:10] ------------------------------------------
---
[01:42:10] test result: FAILED. 9 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:42:10] 
[01:42:10] 
[01:42:10] 
[01:42:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:10] 
[01:42:10] 
[01:42:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:10] Build completed unsuccessfully in 0:55:50
[01:42:10] Build completed unsuccessfully in 0:55:50
[01:42:10] make: *** [check] Error 1
[01:42:10] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:007541b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Nov 24 02:02:18 UTC 2018
---
travis_time:end:14eb2f58:start=1543024941449845534,finish=1543024941454854194,duration=5008660
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12cfc3af
$ ln -s . checkout && for CORE in obj/cores/core.*; 
