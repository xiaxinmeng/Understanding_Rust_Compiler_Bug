plain
travis_time:end:2facbd8b:start=1555974372802925812,finish=1555974375196665459,duration=2393739647
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:37] .................................................................................................... 500/2954
[01:09:49] .................................................................................................... 600/2954
[01:10:04] .................................................................................................... 700/2954
[01:10:15] .................................................................................................... 800/2954
[01:10:25] .............................................F...................................................... 900/2954
[01:10:53] .................................................................................................... 1100/2954
[01:11:03] .................................................................................................... 1200/2954
[01:11:13] .................................................................................................... 1300/2954
[01:11:26] ......................ii............................................................................ 1400/2954
---
[01:15:27] failures:
[01:15:27] 
[01:15:27] ---- [run-pass] run-pass/generator/issue-58888.rs stdout ----
[01:15:27] 
[01:15:27] error: test compilation failed although it shouldn't!
[01:15:27] status: exit code: 101
[01:15:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/generator/issue-58888.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/issue-58888/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/generator/issue-58888/auxiliary"
[01:15:27] ------------------------------------------
[01:15:27] 
[01:15:27] ------------------------------------------
[01:15:27] stderr:
[01:15:27] stderr:
[01:15:27] ------------------------------------------
[01:15:27] error: internal compiler error: src/librustc/ty/mod.rs:2997: item_name: no name for DefPath { data: [DisambiguatedDefPathData { data: Impl, disambiguator: 0 }, DisambiguatedDefPathData { data: ValueNs("check_connection"), disambiguator: 0 }, DisambiguatedDefPathData { data: ClosureExpr, disambiguator: 0 }], krate: crate0 }
[01:15:27] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:636:9
[01:15:27] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:27] error: aborting due to previous error
[01:15:27] 
[01:15:27] 
[01:15:27] 
[01:15:27] note: the compiler unexpectedly panicked. this is a bug.
[01:15:27] 
[01:15:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:15:27] 
[01:15:27] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:15:27] 
[01:15:27] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:15:27] 
[01:15:27] ------------------------------------------
[01:15:27] 
[01:15:27] 
---
[01:15:27] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:15:27] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:27] 
[01:15:27] 
[01:15:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:27] 
[01:15:27] 
[01:15:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:27] Build completed unsuccessfully in 0:11:16
[01:15:27] Build completed unsuccessfully in 0:11:16
[01:15:27] Makefile:48: recipe for target 'check' failed
[01:15:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0181dec1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 00:21:53 UTC 2019
---
travis_time:end:095a1f18:start=1555978915077102049,finish=1555978915081771365,duration=4669316
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:331999ea
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19970612
travis_time:start:19970612
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2dad3df0
$ dmesg | grep -i kill
