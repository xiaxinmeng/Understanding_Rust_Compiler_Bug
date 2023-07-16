plain
travis_time:end:0a6dc973:start=1554634953851432872,finish=1554634956273285309,duration=2421852437
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:41] 
[01:15:41] running 9 tests
[01:15:41] iiiiiiiii
[01:15:41] 
[01:15:41]  finished in 0.160
[01:15:41] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:58] 
[01:15:58] running 121 tests
[01:16:24] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:16:28] i.i......iii.i.....ii
[01:16:28] 
[01:16:28]  finished in 30.742
[01:16:28] travis_fold:end:test_debuginfo

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:47:55] 
[01:47:55] running 24 tests
[01:48:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:48:10] ...........F............
[01:48:10] 
[01:48:10] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:48:10] diff of stdout:
[01:48:10] 
[01:48:10] 
[01:48:10] 15 error: aborting due to previous error
[01:48:10] 16 
[01:48:10] 17 For more information about this error, try `rustc --explain E0425`.
[01:48:10] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:48:10] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:311:13
[01:48:10] 20 
[01:48:10] 20 
[01:48:10] 21 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:48:10] 
[01:48:10] 24 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:48:10] 26 
[01:48:10] - ', src/librustdoc/test.rs:332:17
[01:48:10] + ', src/librustdoc/test.rs:333:17
[01:48:10] 28 
[01:48:10] 28 
[01:48:10] 29 
[01:48:10] 30 failures:
[01:48:10] 
[01:48:10] 
[01:48:10] The actual stdout differed from the expected stdout.
[01:48:10] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:48:10] To update references, rerun the tests and pass the `--bless` flag
[01:48:10] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:48:10] error: 1 errors occurred comparing output.
[01:48:10] status: exit code: 101
[01:48:10] status: exit code: 101
[01:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:48:10] ------------------------------------------
[01:48:10] 
[01:48:10] running 2 tests
[01:48:10] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:48:10] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:48:10] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:48:10] failures:
[01:48:10] 
[01:48:10] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:48:10] error[E0425]: cannot find value `no` in this scope
---
[01:48:10] For more information about this error, try `rustc --explain E0425`.
[01:48:10] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:311:13
[01:48:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:48:10] 
[01:48:10] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:48:10] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:48:10] 
[01:48:10] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:48:10] 
[01:48:10] ', src/librustdoc/test.rs:333:17
[01:48:10] 
[01:48:10] 
[01:48:10] 
[01:48:10] failures:
[01:48:10]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:48:10]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:48:10] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:48:10] 
[01:48:10] 
[01:48:10] ------------------------------------------
---
[01:48:10] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:48:10] 
[01:48:10] 
[01:48:10] 
[01:48:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:48:10] 
[01:48:10] 
[01:48:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:10] Build completed unsuccessfully in 0:44:48
[01:48:10] Build completed unsuccessfully in 0:44:48
[01:48:10] make: *** [check] Error 1
[01:48:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02a66b28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr  7 12:50:58 UTC 2019
---
travis_time:end:1e871428:start=1554641460248615630,finish=1554641460254232538,duration=5616908
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:090ecd58
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00864ed0
travis_time:start:00864ed0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c482e70
$ dmesg | grep -i kill
