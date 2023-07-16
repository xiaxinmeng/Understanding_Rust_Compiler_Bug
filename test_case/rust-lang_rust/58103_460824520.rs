plain
travis_time:end:1c918220:start=1549400438115776704,finish=1549400552148568528,duration=114032791824
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:20] 
[01:09:20] running 38 tests
[01:09:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:09:39] .........F.F...............F..........
[01:09:39] 
[01:09:39] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:09:39] 
[01:09:39] error: compilation failed!
[01:09:39] error: compilation failed!
[01:09:39] status: exit code: 101
[01:09:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/inline-closure-borrows-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure-borrows-arg" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure-borrows-arg/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span_free_formats" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure-borrows-arg/auxiliary"
[01:09:39] ------------------------------------------
[01:09:39] 
[01:09:39] ------------------------------------------
[01:09:39] stderr:
---
[01:09:39] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:39] 
[01:09:39] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:39] 
[01:09:39] note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure-borrows-arg -Z unstable-options -Z span_free_formats -C prefer-dynamic -C rpath
[01:09:39] 
[01:09:39] ------------------------------------------
[01:09:39] 
[01:09:39] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:39] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:09:39] 
[01:09:39] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:09:39] 
[01:09:39] error: compilation failed!
[01:09:39] status: exit code: 101
[01:09:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/inline-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span_free_formats" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure/auxiliary"
[01:09:39] ------------------------------------------
[01:09:39] 
[01:09:39] ------------------------------------------
[01:09:39] stderr:
---
[01:09:39] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:39] 
[01:09:39] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:39] 
[01:09:39] note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline-closure -Z unstable-options -Z span_free_formats -C prefer-dynamic -C rpath
[01:09:39] 
[01:09:39] ------------------------------------------
[01:09:39] 
[01:09:39] thread '[mir-opt] mir-opt/inline-closure.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:39] thread '[mir-opt] mir-opt/inline-closure.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:09:39] 
[01:09:39] ---- [mir-opt] mir-opt/retag.rs stdout ----
[01:09:39] 
[01:09:39] error: compilation failed!
[01:09:39] status: exit code: 101
[01:09:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/retag.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retag" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retag/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-emit-retag" "-Z" "mir-opt-level=0" "-Z" "span_free_formats" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retag/auxiliary"
[01:09:39] ------------------------------------------
[01:09:39] 
[01:09:39] ------------------------------------------
[01:09:39] stderr:
---
[01:09:39] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:39] 
[01:09:39] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:39] 
[01:09:39] note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=3 -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retag -Z unstable-options -Z mir-emit-retag -Z mir-opt-level=0 -Z span_free_formats -C prefer-dynamic -C rpath
[01:09:39] 
[01:09:39] ------------------------------------------
[01:09:39] 
[01:09:39] thread '[mir-opt] mir-opt/retag.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:09:39] test result: FAILED. 35 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:09:39] 
[01:09:39] 
[01:09:39] 
[01:09:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:39] 
[01:09:39] 
[01:09:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:39] Build completed unsuccessfully in 0:11:30
[01:09:39] Build completed unsuccessfully in 0:11:30
[01:09:39] make: *** [check] Error 1
[01:09:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0617ac48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 22:12:20 UTC 2019
---
travis_time:end:23549ac0:start=1549404741764474738,finish=1549404741769243653,duration=4768915
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:178a6a70
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00012129
travis_time:start:00012129
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a907da0
$ dmesg | grep -i kill
