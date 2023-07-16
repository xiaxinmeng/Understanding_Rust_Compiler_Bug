plain
travis_time:end:2b418900:start=1550410914287996784,finish=1550411015276513680,duration=100988516896
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
[01:15:58] 
[01:15:58] running 119 tests
[01:16:25] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:16:29] i......iii.i.....ii
[01:16:29] 
[01:16:29]  finished in 30.676
[01:16:29] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:46:31] 
[01:46:31] running 15 tests
[01:46:32] ...iiiiiiiii...
[01:46:32] 
[01:46:32]  finished in 0.589
[01:46:32] travis_fold:end:test_run-make

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:46:32] 
[01:46:32] running 14 tests
[01:46:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:46:37] ...F..........
[01:46:37] 
[01:46:37] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:46:37] diff of stdout:
[01:46:37] 
[01:46:37] 
[01:46:37] 12 3 | no
[01:46:37] 13   | ^^ not found in this scope
[01:46:37] 14 
[01:46:37] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:46:37] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:359:13
[01:46:37] 17 
[01:46:37] 17 
[01:46:37] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:46:37] 
[01:46:37] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:46:37] 23 
[01:46:37] - ', src/librustdoc/test.rs:389:17
[01:46:37] + ', src/librustdoc/test.rs:394:17
[01:46:37] 25 
[01:46:37] 25 
[01:46:37] 26 
[01:46:37] 27 failures:
[01:46:37] 
[01:46:37] 
[01:46:37] The actual stdout differed from the expected stdout.
[01:46:37] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:46:37] To update references, rerun the tests and pass the `--bless` flag
[01:46:37] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:46:37] error: 1 errors occurred comparing output.
[01:46:37] status: exit code: 101
[01:46:37] status: exit code: 101
[01:46:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:46:37] ------------------------------------------
[01:46:37] 
[01:46:37] running 2 tests
[01:46:37] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:46:37] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:46:37] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:46:37] failures:
[01:46:37] 
[01:46:37] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:46:37] error[E0425]: cannot find value `no` in this scope
---
[01:46:37] 
[01:46:37] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:359:13
[01:46:37] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:46:37] 
[01:46:37] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:46:37] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:46:37] 
[01:46:37] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:46:37] 
[01:46:37] ', src/librustdoc/test.rs:394:17
[01:46:37] 
[01:46:37] 
[01:46:37] 
[01:46:37] failures:
[01:46:37]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:46:37]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:46:37] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:46:37] 
[01:46:37] 
[01:46:37] ------------------------------------------
---
[01:46:37] test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:46:37] 
[01:46:37] 
[01:46:37] 
[01:46:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:46:37] 
[01:46:37] 
[01:46:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:37] Build completed unsuccessfully in 0:42:51
[01:46:37] Build completed unsuccessfully in 0:42:51
[01:46:37] make: *** [check] Error 1
[01:46:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0937a3cc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 15:30:22 UTC 2019
