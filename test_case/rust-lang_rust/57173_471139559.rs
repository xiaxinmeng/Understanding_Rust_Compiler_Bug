plain
travis_time:end:20ef865f:start=1552093532761675960,finish=1552093535086277750,duration=2324601790
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:34] 
[01:19:34] running 119 tests
[01:19:59] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:20:03] i......iii.i.....ii
[01:20:03] 
[01:20:03]  finished in 29.099
[01:20:03] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:49:25] 
[01:49:25] running 15 tests
[01:49:26] ..iiiiiii.ii...
[01:49:26] 
[01:49:26]  finished in 0.632
[01:49:26] travis_fold:end:test_run-make

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:49:26] 
[01:49:26] running 16 tests
[01:49:33] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:49:33] ...F............
[01:49:33] 
[01:49:33] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:49:33] diff of stdout:
[01:49:33] 
[01:49:33] 
[01:49:33] 12 3 | no
[01:49:33] 13   | ^^ not found in this scope
[01:49:33] 14 
[01:49:33] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:352:13
[01:49:33] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:49:33] 17 
[01:49:33] 17 
[01:49:33] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:49:33] 
[01:49:33] The actual stdout differed from the expected stdout.
[01:49:33] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:49:33] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:49:33] To update references, rerun the tests and pass the `--bless` flag
[01:49:33] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:49:33] error: 1 errors occurred comparing output.
[01:49:33] status: exit code: 101
[01:49:33] status: exit code: 101
[01:49:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:49:33] ------------------------------------------
[01:49:33] 
[01:49:33] running 2 tests
[01:49:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:49:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:49:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:49:33] failures:
[01:49:33] 
[01:49:33] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:49:33] error[E0425]: cannot find value `no` in this scope
---
[01:49:33] 
[01:49:33] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:49:33] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:49:33] 
[01:49:33] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:49:33] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:49:33] 
[01:49:33] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:49:33] 
[01:49:33] ', src/librustdoc/test.rs:378:17
[01:49:33] 
[01:49:33] 
[01:49:33] 
[01:49:33] failures:
[01:49:33]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:49:33]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:49:33] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:49:33] 
[01:49:33] 
[01:49:33] ------------------------------------------
---
[01:49:33] test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:49:33] 
[01:49:33] 
[01:49:33] 
[01:49:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:49:33] 
[01:49:33] 
[01:49:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:49:33] Build completed unsuccessfully in 0:41:31
[01:49:33] Build completed unsuccessfully in 0:41:31
[01:49:33] Makefile:48: recipe for target 'check' failed
[01:49:33] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0854dc80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  9 02:55:19 UTC 2019
---
travis_time:end:09ed8118:start=1552100121105992777,finish=1552100121165689553,duration=59696776
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e8ee0b3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:099fd960
$ dmesg | grep -i kill
