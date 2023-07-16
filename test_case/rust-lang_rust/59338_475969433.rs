plain
travis_time:end:285a3888:start=1553431900345291414,finish=1553431976696972248,duration=76351680834
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
[02:00:35] 
[02:00:35] running 9 tests
[02:00:35] iiiiiiiii
[02:00:35] 
[02:00:35]  finished in 0.166
[02:00:35] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[02:00:53] 
[02:00:53] running 120 tests
[02:01:23] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[02:01:28] .i......iii.i.....ii
[02:01:28] 
[02:01:28]  finished in 35.704
[02:01:28] travis_fold:end:test_debuginfo

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[02:31:38] 
[02:31:38] running 24 tests
[02:31:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[02:31:50] ...........F............
[02:31:50] 
[02:31:50] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[02:31:50] diff of stdout:
[02:31:50] 
[02:31:50] 
[02:31:50] 15 error: aborting due to previous error
[02:31:50] 16 
[02:31:50] 17 For more information about this error, try `rustc --explain E0425`.
[02:31:50] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[02:31:50] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:308:13
[02:31:50] 20 
[02:31:50] 20 
[02:31:50] 21 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[02:31:50] 
[02:31:50] 24 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[02:31:50] 26 
[02:31:50] - ', src/librustdoc/test.rs:332:17
[02:31:50] + ', src/librustdoc/test.rs:330:17
[02:31:50] 28 
[02:31:50] 28 
[02:31:50] 29 
[02:31:50] 30 failures:
[02:31:50] 
[02:31:50] 
[02:31:50] The actual stdout differed from the expected stdout.
[02:31:50] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[02:31:50] To update references, rerun the tests and pass the `--bless` flag
[02:31:50] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[02:31:50] error: 1 errors occurred comparing output.
[02:31:50] status: exit code: 101
[02:31:50] status: exit code: 101
[02:31:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[02:31:50] ------------------------------------------
[02:31:50] 
[02:31:50] running 2 tests
[02:31:50] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[02:31:50] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[02:31:50] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[02:31:50] failures:
[02:31:50] 
[02:31:50] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[02:31:50] error[E0425]: cannot find value `no` in this scope
---
[02:31:50] For more information about this error, try `rustc --explain E0425`.
[02:31:50] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:308:13
[02:31:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[02:31:50] 
[02:31:50] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[02:31:50] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[02:31:50] 
[02:31:50] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[02:31:50] 
[02:31:50] ', src/librustdoc/test.rs:330:17
[02:31:50] 
[02:31:50] 
[02:31:50] 
[02:31:50] failures:
[02:31:50]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[02:31:50]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[02:31:50] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[02:31:50] 
[02:31:50] 
[02:31:50] ------------------------------------------
---
[02:31:50] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[02:31:50] 
[02:31:50] 
[02:31:50] 
[02:31:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:31:50] 
[02:31:50] 
[02:31:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[02:31:50] Build completed unsuccessfully in 0:44:17
[02:31:50] Build completed unsuccessfully in 0:44:17
[02:31:50] make: *** [check] Error 1
[02:31:50] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21986fda
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 24 15:24:57 UTC 2019
---
travis_time:end:1d3771d0:start=1553441099899780364,finish=1553441099905426896,duration=5646532
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09b84fbd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05fc11ba
travis_time:start:05fc11ba
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1dffd9a9
$ dmesg | grep -i kill
