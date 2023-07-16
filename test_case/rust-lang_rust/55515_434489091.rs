plain
travis_time:end:199bc3a2:start=1540931077754171562,finish=1540931148860112008,duration=71105940446
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:51:17] .................................................................................................... 100/4983
[00:51:20] .................................................................................................... 200/4983
[00:51:23] ...........................................................................................ii....... 300/4983
[00:51:26] .........................................................................................iii........ 400/4983
[00:51:29] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:51:36] .................................................................................................... 700/4983
[00:51:42] ..................................................................i...........i..................... 800/4983
[00:51:45] ....................................................................................iiiii........... 900/4983
[00:51:49] .................................................................................................... 1000/4983
---
[00:52:28] .................................................................................................... 2200/4983
[00:52:32] .................................................................................................... 2300/4983
[00:52:36] .................................................................................................... 2400/4983
[00:52:40] .................................................................................................... 2500/4983
[00:52:44] ...................................................................iiiiiiiii........................ 2600/4983
[00:52:51] ..................ii................................................................................ 2800/4983
[00:52:54] .................................................................................................... 2900/4983
[00:52:58] .................................................................................................... 3000/4983
[00:53:01] .............i...................................................................................... 3100/4983
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:30] 
[01:06:30] running 112 tests
[01:06:33] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:06:34] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:06:34] 
[01:06:34]  finished in 3.612
[01:06:34] travis_fold:end:test_codegen
---
[01:46:33] 
[01:46:33] 12 3 | no
[01:46:33] 13   | ^^ not found in this scope
[01:46:33] 14 
[01:46:33] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:46:33] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:46:33] 17 
[01:46:33] 17 
[01:46:33] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:46:33] 
[01:46:33] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:46:33] 23 
[01:46:33] - ', librustdoc/test.rs:367:17
[01:46:33] + ', librustdoc/test.rs:358:17
[01:46:33] 25 
[01:46:33] 25 
[01:46:33] 26 
[01:46:33] 27 failures:
[01:46:33] 
[01:46:33] 
[01:46:33] The actual stdout differed from the expected stdout.
[01:46:33] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:46:33] To update references, rerun the tests and pass the `--bless` flag
[01:46:33] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:46:33] error: 1 errors occurred comparing output.
[01:46:33] status: exit code: 101
[01:46:33] status: exit code: 101
[01:46:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:46:33] ------------------------------------------
[01:46:33] 
[01:46:33] running 2 tests
[01:46:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:46:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) ... FAILED
[01:46:33] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) ... FAILED
[01:46:33] failures:
[01:46:33] 
[01:46:33] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:46:33] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27) stdout ----
[01:46:33] error[E0425]: cannot find value `no` in this scope
[01:46:33]   |
[01:46:33] 3 | no
[01:46:33]   | ^^ not found in this scope
[01:46:33] 
[01:46:33] 
[01:46:33] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:46:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:46:33] 
[01:46:33] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:46:33] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)' panicked at 'test executable failed:
[01:46:33] 
[01:46:33] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:46:33] 
[01:46:33] ', librustdoc/test.rs:358:17
[01:46:33] 
[01:46:33] 
[01:46:33] 
[01:46:33] failures:
[01:46:33]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 27)
[01:46:33]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 21)
[01:46:33] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:46:33] 
[01:46:33] 
[01:46:33] ------------------------------------------
---
[01:46:33] test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:46:33] 
[01:46:33] 
[01:46:33] 
[01:46:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options " "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:46:33] 
[01:46:33] 
[01:46:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:33] Build completed unsuccessfully in 0:59:06
[01:46:33] Build completed unsuccessfully in 0:59:06
[01:46:33] Makefile:58: recipe for target 'check' failed
[01:46:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15961758
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:152e5fbc:start=1540937563897119088,finish=1540937563903988632,duration=6869544
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:086a87cc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
