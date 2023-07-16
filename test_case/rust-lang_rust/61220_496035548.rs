plain
travis_time:end:001ec79c:start=1558904110274822038,finish=1558904111024944124,duration=750122086
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:31] 
[01:11:31] running 5589 tests
[01:11:34] .........................................................................F..........F............... 100/5589
[01:11:44] .................................................................................................... 300/5589
[01:11:47] .................................................................................................... 400/5589
[01:11:51] ...................................................................................................i 500/5589
[01:11:55] .................................................................................................... 600/5589
---
[01:12:30] .................................................................................................... 1600/5589
[01:12:33] .............................................i...................................................... 1700/5589
[01:12:37] .................................................................................................... 1800/5589
[01:12:41] .................................................................................................... 1900/5589
[01:12:44] ..............................................................................F..................... 2000/5589
[01:12:52] .................................................................................................... 2200/5589
[01:12:56] .................................................................................................... 2300/5589
[01:13:00] .................................................................................................... 2400/5589
[01:13:05] .................................................................................................... 2500/5589
---
[01:14:01] ....................i............................................................................... 4000/5589
[01:14:03] ....................................................................................i............... 4100/5589
[01:14:05] .................................................................................................... 4200/5589
[01:14:10] .................................................................................................... 4300/5589
[01:14:20] .............................................................................F...................... 4400/5589
[01:14:27] .................................................................................................... 4600/5589
[01:14:31] .................................................................................................... 4700/5589
[01:14:38] .................................................................................................... 4800/5589
[01:14:44] .................................................................................................... 4900/5589
---
[01:15:09] 
[01:15:09] 
[01:15:09] The actual stderr differed from the expected stderr.
[01:15:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding/associated-types-overridden-binding.stderr
[01:15:09] To update references, rerun the tests and pass the `--bless` flag
[01:15:09] To only update this specific test, also pass `--test-args associated-types/associated-types-overridden-binding.rs`
[01:15:09] error: 1 errors occurred comparing output.
[01:15:09] status: exit code: 1
[01:15:09] status: exit code: 1
[01:15:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding/auxiliary" "-A" "unused"
[01:15:09] ------------------------------------------
[01:15:09] 
[01:15:09] ------------------------------------------
[01:15:09] stderr:
[01:15:09] stderr:
[01:15:09] ------------------------------------------
[01:15:09] error[E0284]: type annotations required: cannot resolve `<Self as std::iter::Iterator>::Item == i32`
[01:15:09]    |
[01:15:09]    |
[01:15:09] LL | trait Bar: Foo<Item = u32> {} //~ ERROR type annotations required
[01:15:09]    |
[01:15:09] note: required by `Foo`
[01:15:09]   --> /checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs:3:1
[01:15:09]    |
[01:15:09]    |
[01:15:09] LL | trait Foo: Iterator<Item = i32> {}
[01:15:09] 
[01:15:09] error: aborting due to previous error
[01:15:09] 
[01:15:09] For more information about this error, try `rustc --explain E0284`.
---
[01:15:09] 9 
[01:15:09] 
[01:15:09] 
[01:15:09] The actual stderr differed from the expected stderr.
[01:15:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/associated-types-unconstrained.stderr
[01:15:09] To update references, rerun the tests and pass the `--bless` flag
[01:15:09] To only update this specific test, also pass `--test-args associated-types/associated-types-unconstrained.rs`
[01:15:09] error: 1 errors occurred comparing output.
[01:15:09] status: exit code: 1
[01:15:09] status: exit code: 1
[01:15:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-unconstrained.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/auxiliary" "-A" "unused"
[01:15:09] ------------------------------------------
[01:15:09] 
[01:15:09] ------------------------------------------
[01:15:09] stderr:
[01:15:09] stderr:
[01:15:09] ------------------------------------------
[01:15:09] error[E0284]: type annotations required: cannot resolve `<_ as Foo>::A == _`
[01:15:09]    |
[01:15:09]    |
[01:15:09] LL |     let x: isize = Foo::bar();
[01:15:09] 
[01:15:09] error: aborting due to previous error
[01:15:09] 
[01:15:09] For more information about this error, try `rustc --explain E0284`.
---
[01:15:09] 
[01:15:09] 
[01:15:09] The actual stderr differed from the expected stderr.
[01:15:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/issue-12028.stderr
[01:15:09] To update references, rerun the tests and pass the `--bless` flag
[01:15:09] To only update this specific test, also pass `--test-args issues/issue-12028.rs`
[01:15:09] error: 1 errors occurred comparing output.
[01:15:09] status: exit code: 1
[01:15:09] status: exit code: 1
[01:15:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12028.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12028/auxiliary" "-A" "unused"
[01:15:09] ------------------------------------------
[01:15:09] 
[01:15:09] ------------------------------------------
[01:15:09] stderr:
[01:15:09] stderr:
[01:15:09] ------------------------------------------
[01:15:09] error[E0284]: type annotations required: cannot resolve `<_ as StreamHasher>::S == <H as StreamHasher>::S`
[01:15:09]    |
[01:15:09]    |
[01:15:09] LL |         self.input_stream(&mut stream); //~ ERROR type annotations required
[01:15:09] 
[01:15:09] error: aborting due to previous error
[01:15:09] 
[01:15:09] For more information about this error, try `rustc --explain E0284`.
---
[01:15:09] 9 
[01:15:09] 
[01:15:09] 
[01:15:09] The actual stderr differed from the expected stderr.
[01:15:09] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer/question-mark-type-infer.stderr
[01:15:09] To update references, rerun the tests and pass the `--bless` flag
[01:15:09] To only update this specific test, also pass `--test-args question-mark-type-infer.rs`
[01:15:09] error: 1 errors occurred comparing output.
[01:15:09] status: exit code: 1
[01:15:09] status: exit code: 1
[01:15:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/question-mark-type-infer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer/auxiliary" "-A" "unused"
[01:15:09] ------------------------------------------
[01:15:09] 
[01:15:09] ------------------------------------------
[01:15:09] stderr:
[01:15:09] stderr:
[01:15:09] ------------------------------------------
[01:15:09] error[E0284]: type annotations required: cannot resolve `<_ as std::ops::Try>::Ok == _`
[01:15:09]   --> /checkout/src/test/ui/question-mark-type-infer.rs:12:5
[01:15:09]    |
[01:15:09] LL |     l.iter().map(f).collect()? //~ ERROR type annotations required: cannot resolve
[01:15:09] 
[01:15:09] error: aborting due to previous error
[01:15:09] 
[01:15:09] For more information about this error, try `rustc --explain E0284`.
---
[01:15:09] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:15:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:09] 
[01:15:09] 
[01:15:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:09] 
[01:15:09] 
[01:15:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:09] Build completed unsuccessfully in 0:04:55
[01:15:09] Build completed unsuccessfully in 0:04:55
[01:15:09] Makefile:48: recipe for target 'check' failed
[01:15:09] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2232d37a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 22:10:30 UTC 2019
---
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1e268ea3
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: ‘/home/travis/Library/Logs/DiagnosticReports’: No such file or directory
travis_time:end:1e268ea3:startravis_time:start:06359f10
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04854aad
travis_time:start:04854aad
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a260efb
$ dmesg | grep -i kill
