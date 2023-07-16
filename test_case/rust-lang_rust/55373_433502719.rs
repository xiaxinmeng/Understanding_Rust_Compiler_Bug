plain
travis_time:end:29b35abc:start=1540575235567081098,finish=1540575290297513653,duration=54730432555
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:49:20] .................................................................................................... 2200/4955
[00:49:24] .................................................................................................... 2300/4955
[00:49:27] .................................................................................................... 2400/4955
[00:49:31] .................................................................................................... 2500/4955
[00:49:34] .......................................................iiiiiiiii.................................... 2600/4955
[00:49:41] .....ii............................................................................................. 2800/4955
[00:49:44] .................................................................................................... 2900/4955
[00:49:47] ..................................................................................................i. 3000/4955
[00:49:50] .................................................................................................... 3100/4955
---
[00:53:05] ........................................................................i........................... 1400/2869
[00:53:16] .................................................................................................... 1500/2869
[00:53:27] .........................................i.......................................................... 1600/2869
[00:53:41] .................................................................................................... 1700/2869
[00:53:52] ................................................F................................................... 1800/2869
[00:54:13] .....................................i.............................................................. 2000/2869
[00:54:34] .................................................................................................... 2100/2869
[00:54:40] ...................................................................................................i 2200/2869
[00:54:56] i.....................................................................i....i........................ 2300/2869
---
[00:56:15] failures:
[00:56:15] 
[00:56:15] ---- [run-pass] run-pass/macros/macro-first-set.rs stdout ----
[00:56:15] 
[00:56:15] error: test compilation failed although it shouldn't!
[00:56:15] status: exit code: 1
[00:56:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/macro-first-set.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-first-set/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-first-set/auxiliary"
[00:56:15] ------------------------------------------
[00:56:15] 
[00:56:15] ------------------------------------------
[00:56:15] stderr:
[00:56:15] stderr:
[00:56:15] ------------------------------------------
[00:56:15] {"message":"`$arg:pat` is followed (through repetition) by itself, which is not allowed for `pat` fragments","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/macros/macro-first-set.rs","byte_start":1194,"byte_end":1202,"line_start":43,"line_end":43,"column_start":21,"column_end":29,"is_primary":true,"text":[{"text":"    ($fname:ident $($arg:pat)* =) => {}","highlight_start":21,"highlight_end":29}],"label":"this fragment is followed by itself without a valid separator","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `$arg:pat` is followed (through repetition) by itself, which is not allowed for `pat` fragments\n  --> /checkout/src/test/run-pass/macros/macro-first-set.rs:43:21\n   |\nLL |     ($fname:ident $($arg:pat)* =) => {}\n   |                     ^^^^^^^^ this fragment is followed by itself without a valid separator\n\n"}
[00:56:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:15] ------------------------------------------
[00:56:15] 
[00:56:15] thread '[run-pass] run-pass/macros/macro-first-set.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:56:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:15] 
[00:56:15] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:56:15] 
[00:56:15] 
[00:56:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:15] 
[00:56:15] 
[00:56:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
travis_time:end:0f103400:start=1540575300221661696,finish=1540578676073297741,duration=3375851636045
