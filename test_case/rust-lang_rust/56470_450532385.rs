plain
travis_time:end:03c9b48a:start=1546127737805862534,finish=1546127738817457861,duration=1011595327
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
[01:10:08] 
[01:10:08] running 118 tests
[01:10:32] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:10:36] ......iii.i.....ii
[01:10:36] 
[01:10:36]  finished in 28.114
[01:10:36] travis_fold:end:test_debuginfo

---
[01:15:55] 
[01:15:55] running 289 tests
[01:17:03] ..........................i......................................................................... 100/289
[01:17:58] .....................................i.............................................................. 200/289
[01:18:49] ...............................F.........................................................
[01:18:49] 
[01:18:49] ---- [rustdoc] rustdoc/process-termination.rs stdout ----
[01:18:49] 
[01:18:49] error: rustdoc failed!
[01:18:49] error: rustdoc failed!
[01:18:49] status: exit code: 101
[01:18:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
[01:18:49] ------------------------------------------
[01:18:49] 
[01:18:49] running 4 tests
[01:18:49] test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 29) ... FAILED
---
[01:18:49] ---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 29) stdout ----
[01:18:49] error[E0282]: type annotations needed
[01:18:49]  --> /checkout/src/test/rustdoc/process-termination.rs:30:1
[01:18:49]   |
[01:18:49] 3 | Err::<(), &'static str>("This is returned from `main`, leading to panic")?;
[01:18:49] 
[01:18:49] thread '/checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 29)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:316:13
[01:18:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:49] 
---
[01:18:49] 
[01:18:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:18:49] 
[01:18:49] 
[01:18:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:49] 
[01:18:49] 
[01:18:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:49] Build completed unsuccessfully in 0:19:58
[01:18:49] Build completed unsuccessfully in 0:19:58
[01:18:49] Makefile:48: recipe for target 'check' failed
[01:18:49] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16648198
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 30 01:14:37 UTC 2018
---
travis_time:end:0fab0c04:start=1546132479878845964,finish=1546132479887099637,duration=8253673
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cf88bec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ];
