plain
travis_time:end:13dfc8ef:start=1556107976882379601,finish=1556107979048841999,duration=2166462398
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:19] .............................................ii..................................................... 3800/5463
[01:09:21] ...............................................................i.................................... 3900/5463
[01:09:24] .................................................................................................... 4000/5463
[01:09:26] .......................i............................................................................ 4100/5463
[01:09:29] .........................................F.......................................................... 4200/5463
[01:09:43] .................................................................................................... 4400/5463
[01:09:46] .................................................................................................... 4500/5463
[01:09:50] .................................................................................................... 4600/5463
[01:09:56] .................................................................................................... 4700/5463
---
[01:10:21] 
[01:10:21] ---- [ui] ui/consts/const_let_refutable.rs stdout ----
[01:10:21] diff of stderr:
[01:10:21] 
[01:10:21] 4 LL | const fn slice([a, b]: &[i32]) -> i32 {
[01:10:21] 5    |                ^^^^^^ pattern `&[]` not covered
[01:10:21] 6 
[01:10:21] - error[E0723]: can only call other `const fn` within a `const fn`, but `const std::ops::Add::add` is not stable as `const fn`
[01:10:21] + error[E0723]: can only call other `const fn` within a `const fn`, but `const <&i32 as std::ops::Add>::add` is not stable as `const fn`
[01:10:21] 9    |
[01:10:21] 10 LL |     a + b
[01:10:21] 
[01:10:21] 
[01:10:21] 
[01:10:21] The actual stderr differed from the expected stderr.
[01:10:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable/const_let_refutable.stderr
[01:10:21] To update references, rerun the tests and pass the `--bless` flag
[01:10:21] To only update this specific test, also pass `--test-args consts/const_let_refutable.rs`
[01:10:21] error: 1 errors occurred comparing output.
[01:10:21] status: exit code: 1
[01:10:21] status: exit code: 1
[01:10:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_let_refutable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable/auxiliary" "-A" "unused"
[01:10:21] ------------------------------------------
[01:10:21] 
[01:10:21] ------------------------------------------
[01:10:21] stderr:
[01:10:21] stderr:
[01:10:21] ------------------------------------------
[01:10:21] error[E0005]: refutable pattern in function argument: `&[]` not covered
[01:10:21]    |
[01:10:21]    |
[01:10:21] LL | const fn slice([a, b]: &[i32]) -> i32 { //~ ERROR refutable pattern in function argument
[01:10:21]    |                ^^^^^^ pattern `&[]` not covered
[01:10:21] 
[01:10:21] error[E0723]: can only call other `const fn` within a `const fn`, but `const <&i32 as std::ops::Add>::add` is not stable as `const fn`
[01:10:21]    |
[01:10:21]    |
[01:10:21] LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
[01:10:21]    |
[01:10:21]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:10:21]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:10:21] 
[01:10:21] 
[01:10:21] warning[E0381]: use of possibly uninitialized variable: `a`
[01:10:21]   --> /checkout/src/test/ui/consts/const_let_refutable.rs:4:5
[01:10:21]    |
[01:10:21] LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
[01:10:21]    |     ^ use of possibly uninitialized `a`
[01:10:21]    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:21]    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:21] 
[01:10:21] warning[E0381]: use of possibly uninitialized variable: `b`
[01:10:21] warning[E0381]: use of possibly uninitialized variable: `b`
[01:10:21]   --> /checkout/src/test/ui/consts/const_let_refutable.rs:4:9
[01:10:21]    |
[01:10:21] LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
[01:10:21]    |         ^ use of possibly uninitialized `b`
[01:10:21]    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:10:21]    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:10:21] 
[01:10:21] error: aborting due to 2 previous errors
---
[01:10:21] 
[01:10:21] ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
[01:10:21] diff of stderr:
[01:10:21] 
[01:10:21] - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1071:5
[01:10:21] + thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1073:5
[01:10:21] 3 
[01:10:21] 4 error: internal compiler error: unexpected panic
[01:10:21] 
[01:10:21] 
[01:10:21] 
[01:10:21] The actual stderr differed from the expected stderr.
[01:10:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
[01:10:21] To update references, rerun the tests and pass the `--bless` flag
[01:10:21] To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
[01:10:21] error: 1 errors occurred comparing output.
[01:10:21] status: exit code: 101
[01:10:21] status: exit code: 101
[01:10:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
[01:10:21] ------------------------------------------
[01:10:21] 
[01:10:21] ------------------------------------------
[01:10:21] stderr:
[01:10:21] stderr:
[01:10:21] ------------------------------------------
[01:10:21] thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1073:5
[01:10:21] 
[01:10:21] error: internal compiler error: unexpected panic
[01:10:21] 
[01:10:21] note: the compiler unexpectedly panicked. this is a bug.
[01:10:21] note: the compiler unexpectedly panicked. this is a bug.
[01:10:21] 
[01:10:21] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:21] 
[01:10:21] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:10:21] 
[01:10:21] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:10:21] 
[01:10:21] ------------------------------------------
[01:10:21] 
[01:10:21] 
---
[01:10:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:10:21] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:21] 
[01:10:21] 
[01:10:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:21] 
[01:10:21] 
[01:10:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:21] Build completed unsuccessfully in 0:04:15
[01:10:21] Build completed unsuccessfully in 0:04:15
[01:10:21] make: *** [check] Error 1
[01:10:21] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e249294
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 13:23:32 UTC 2019
---
travis_time:end:03d94ed4:start=1556112213917137820,finish=1556112213921596776,duration=4458956
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a19d580
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:300794a8
travis_time:start:300794a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12a6ff14
$ dmesg | grep -i kill
