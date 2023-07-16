plain
travis_time:end:1c87c808:start=1552083237145324612,finish=1552083239555624791,duration=2410300179
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
[01:22:37] 
[01:22:37] running 119 tests
[01:23:04] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:23:08] i......iii.i.....ii
[01:23:08] 
[01:23:08]  finished in 31.398
[01:23:08] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:54:32] 
[01:54:32] running 15 tests
[01:54:32] ..iiiiiii.ii...
[01:54:32] 
[01:54:32]  finished in 0.693
[01:54:32] travis_fold:end:test_run-make

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:54:33] 
[01:54:33] running 16 tests
[01:54:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:54:40] ...F............
[01:54:40] 
[01:54:40] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:54:40] diff of stdout:
[01:54:40] 
[01:54:40] 
[01:54:40] 12 3 | no
[01:54:40] 13   | ^^ not found in this scope
[01:54:40] 14 
[01:54:40] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:352:13
[01:54:40] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:54:40] 17 
[01:54:40] 17 
[01:54:40] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:54:40] 
[01:54:40] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:54:40] 23 
[01:54:40] - ', src/librustdoc/test.rs:373:17
[01:54:40] + ', src/librustdoc/test.rs:378:17
[01:54:40] 25 
[01:54:40] 25 
[01:54:40] 26 
[01:54:40] 27 failures:
[01:54:40] 
[01:54:40] 
[01:54:40] The actual stdout differed from the expected stdout.
[01:54:40] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:54:40] To update references, rerun the tests and pass the `--bless` flag
[01:54:40] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:54:40] error: 1 errors occurred comparing output.
[01:54:40] status: exit code: 101
[01:54:40] status: exit code: 101
[01:54:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:54:40] ------------------------------------------
[01:54:40] 
[01:54:40] running 2 tests
[01:54:40] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:54:40] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:54:40] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:54:40] failures:
[01:54:40] 
[01:54:40] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:54:40] error[E0425]: cannot find value `no` in this scope
---
[01:54:40] 
[01:54:40] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:357:13
[01:54:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:54:40] 
[01:54:40] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:54:40] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:54:40] 
[01:54:40] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:54:40] 
[01:54:40] ', src/librustdoc/test.rs:378:17
[01:54:40] 
[01:54:40] 
[01:54:40] 
[01:54:40] failures:
[01:54:40]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:54:40]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:54:40] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:54:40] 
[01:54:40] 
[01:54:40] ------------------------------------------
---
[01:54:40] test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:54:40] 
[01:54:40] 
[01:54:40] 
[01:54:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:54:40] 
[01:54:40] 
[01:54:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:54:40] Build completed unsuccessfully in 0:44:18
[01:54:40] Build completed unsuccessfully in 0:44:18
[01:54:40] Makefile:48: recipe for target 'check' failed
[01:54:40] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03c73786
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  9 00:08:50 UTC 2019
---
travis_time:end:037c5d50:start=1552090131858036655,finish=1552090131863634671,duration=5598016
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04aecf50
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
