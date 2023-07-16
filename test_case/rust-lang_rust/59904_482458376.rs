plain
travis_time:end:2598808e:start=1555044430024255961,finish=1555044432178302751,duration=2154046790
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
[01:24:55] 
[01:24:55] running 9 tests
[01:24:55] iiiiiiiii
[01:24:55] 
[01:24:55]  finished in 0.185
[01:24:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:15] 
[01:25:15] running 121 tests
[01:25:45] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:25:51] i.i......iii.i.....ii
[01:25:51] 
[01:25:51]  finished in 36.517
[01:25:51] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:59:18] 
[01:59:18] running 24 tests
[01:59:33] ...........F............
[01:59:33] failures:
[01:59:33] 
[01:59:33] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:59:33] diff of stdout:
[01:59:33] diff of stdout:
[01:59:33] 
[01:59:33] 15 error: aborting due to previous error
[01:59:33] 16 
[01:59:33] 17 For more information about this error, try `rustc --explain E0425`.
[01:59:33] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:308:13
[01:59:33] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:307:13
[01:59:33] 20 
[01:59:33] 20 
[01:59:33] 21 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:59:33] 
[01:59:33] 24 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:59:33] 26 
[01:59:33] - ', src/librustdoc/test.rs:330:17
[01:59:33] + ', src/librustdoc/test.rs:329:17
[01:59:33] 28 
[01:59:33] 28 
[01:59:33] 29 
[01:59:33] 30 failures:
[01:59:33] 
[01:59:33] 
[01:59:33] The actual stdout differed from the expected stdout.
[01:59:33] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:59:33] To update references, rerun the tests and pass the `--bless` flag
[01:59:33] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:59:33] error: 1 errors occurred comparing output.
[01:59:33] status: exit code: 101
[01:59:33] status: exit code: 101
[01:59:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:59:33] ------------------------------------------
[01:59:33] 
[01:59:33] running 2 tests
[01:59:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:59:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:59:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:59:33] failures:
[01:59:33] 
[01:59:33] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:59:33] error[E0425]: cannot find value `no` in this scope
---
[01:59:33] For more information about this error, try `rustc --explain E0425`.
[01:59:33] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:307:13
[01:59:33] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:59:33] 
[01:59:33] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:59:33] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:59:33] 
[01:59:33] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:59:33] 
[01:59:33] ', src/librustdoc/test.rs:329:17
[01:59:33] 
[01:59:33] 
[01:59:33] 
[01:59:33] failures:
[01:59:33]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:59:33]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:59:33] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:59:33] 
[01:59:33] 
[01:59:33] ------------------------------------------
---
[01:59:33] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:59:33] 
[01:59:33] 
[01:59:33] 
[01:59:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:59:33] 
[01:59:33] 
[01:59:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:59:33] Build completed unsuccessfully in 0:48:21
[01:59:33] Build completed unsuccessfully in 0:48:21
[01:59:33] Makefile:48: recipe for target 'check' failed
[01:59:33] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fa32ada
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 12 06:46:57 UTC 2019
---
58644 ./obj/build/x86_64-unknown-linux-gntravis_time:end:037ea182:start=1555051619347875781,finish=1555051619418386983,duration=70511202
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23a3b208
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:003643fe
$ dmesg | grep -i kill
