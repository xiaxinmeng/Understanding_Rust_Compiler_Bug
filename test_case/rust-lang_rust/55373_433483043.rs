plain
travis_time:end:2accea05:start=1540571271866295998,finish=1540571331713735577,duration=59847439579
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:48:59] .................................................................................................... 2200/4955
[00:49:03] .................................................................................................... 2300/4955
[00:49:07] .................................................................................................... 2400/4955
[00:49:10] .................................................................................................... 2500/4955
[00:49:14] .......................................................iiiiiiiii.................................... 2600/4955
[00:49:20] .....ii............................................................................................. 2800/4955
[00:49:23] .................................................................................................... 2900/4955
[00:49:27] ..................................................................................................i. 3000/4955
[00:49:29] .................................................................................................... 3100/4955
---
[00:52:12] .................................................................................................... 1000/2869
[00:52:24] .................................................................................................... 1100/2869
[00:52:33] .................................................................................................... 1200/2869
[00:52:43] .................................................................................................... 1300/2869
[00:52:55] ........................................................................i...........F............... 1400/2869
[00:53:20] .........................................i.......................................................... 1600/2869
[00:53:34] .................................................................................................... 1700/2869
[00:53:34] .................................................................................................... 1700/2869
[00:53:46] ................................................F................................................... 1800/2869
[00:54:09] .....................................i.............................................................. 2000/2869
[00:54:32] .................................................................................................... 2100/2869
[00:54:39] ...................................................................................................i 2200/2869
[00:54:56] i.....................................................................i....i........................ 2300/2869
---
[00:56:18] failures:
[00:56:18] 
[00:56:18] ---- [run-pass] run-pass/issues/issue-33185.rs stdout ----
[00:56:18] 
[00:56:18] error: test compilation failed although it shouldn't!
[00:56:18] status: exit code: 1
[00:56:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-33185.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-33185/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-33185/auxiliary"
[00:56:18] stdout:
[00:56:18] ------------------------------ment in the evaluated repetition\n\n"}
[00:56:18] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:18] ------------------------------------------
[00:56:18] 
[00:56:18] thread '[run-pass] run-pass/issues/issue-33185.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:56:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:18] 
[00:56:18] ---- [run-pass] run-pass/macros/macro-first-set.rs stdout ----
[00:56:18] 
[00:56:18] error: test compilation failed although it shouldn't!
[00:56:18] status: exit code: 1
[00:56:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/macro-first-set.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-first-set/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-first-set/auxiliary"
[00:56:18] ------------------------------------------
[00:56:18] 
[00:56:18] ------------------------------------------
[00:56:18] stderr:
[00:56:18] stderr:
[00:56:18] ------------------------------------------
[00:56:18] {"message":"`$arg:pat` is followed (through repetition) by itself, which is not allowed for `pat` fragments","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/macros/macro-first-set.rs","byte_start":1194,"byte_end":1202,"line_start":43,"line_end":43,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    ($fname:ident $($arg:pat)* =) => {}","highlight_start":21,"highlight_end":29}],"label":"this fragment is followed by itself without a valid separator","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `$arg:pat` is followed (through repetition) by itself, which is not allowed for `pat` fragments\n  --> /checkout/src/test/run-pass/macros/macro-first-set.rs:43:21\n   |\nLL |     ($fname:ident $($arg:pat)* =) => {}\n   |                     ^^^^^^^^ this fragment is followed by itself without a valid separator\n\n"}
[00:56:18] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:18] ------------------------------------------
[00:56:18] 
[00:56:18] thread '[run-pass] run-pass/macros/macro-first-set.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:56:18] 
---
[00:56:18] 
[00:56:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:56:18] 
[00:56:18] 
[00:56:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:18] 
[00:56:18] 
[00:56:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:18] Build completed unsuccessfully in 0:11:58
[00:56:18] Build completed unsuccessfully in 0:11:58
[00:56:18] Makefile:58: recipe for target 'check' failed
[00:56:18] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:062efba2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:022ea39f:start=1540574724432479919,finish=1540574724437323808,duration=4843889
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05e8d7f4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:084f9e07
travis_time:start:084f9e07
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06849b58
$ dmesg | grep -i kill
