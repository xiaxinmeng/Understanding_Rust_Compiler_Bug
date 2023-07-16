plain
Resolving deltas: 100% (607325/607325), completed with 4806 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:44:10] ..........................................................................i.........................
[00:44:17] .................i..................................................................................
---
[00:44:54] .............................................................................................i......
[00:45:02] .................................................................i..................................
---
[00:46:00] .............................................i......................................................
---
[00:50:10] .............................i......................................................................
[00:50:25] ..............................................................i.....................................
[00:50:42] ...............................................i....................................................
[00:51:03] ....................................................................................................
[00:51:26] ....................................................................................................
[00:51:49] ....................................................................................................
[00:52:16] .i...............................................................................................i..
[00:52:41] ....................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:52:55] ................................
[00:53:28] ....................................................................................................
[00:54:07] ..............................................................ii....................................
[00:54:49] .........................i............................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:55:00] ........i.ii..................
[00:55:43] ...............................F......................................................iiiiiii.......
[00:56:14] ....................................................................................................
[00:56:42] ....................................................................................................
[00:57:07] ....................................................................................................
:23] stderr:
[00:57:23] ------------------------------------------
[00:57:23] error[E0460]: found possibly newer version of crate `a` which `b` depends on
[00:57:23]   --> /checkout/src/test/run-pass/svh-add-nothing.rs:19:1
[00:57:23]    |
[00:57:23] 19 | extern crate b;
[00:57:23]    | ^^^^^^^^^^^^^^^
[00:57:23]    |
[00:57:23]    = note: perhaps that crate needs to be recompiled?
[00:57:23]    = note: the following crate versions were found:
[00:57:23]            crate `a`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/svh-add-nothing.stage2-x86_64-unknown-linux-gnu.aux/liba.so
[00:57:23]            crate `b`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/svh-add-nothing.stage2-x86_64-unknown-linux-gnu.aux/libb.so
---
[00:57:23] thread '[run-pass] run-pass/svh-add-nothing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
---
[00:57:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:23] expected success, got: exit code: 101
[00:57:23]
[00:57:23]
[00:57:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:23] Build completed unsuccessfully in 0:14:05
[00:57:23] Makefile:58: recipe for target 'check' failed
[00:57:23] make: *** [check] Error 1
---
56700 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08/s-ezry20nkee-1ezllpu-2cfdg74j6tjst
---
34/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0338468a:start=1522769086835762324,finish=1522769086843587149,duration=7824825
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0141643a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0141643a:start=1522769086850024326,finish=1522769086857617025,duration=7592699
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21947c2e
$ dmesg | grep -i kill
[   10.534524] init: failsafe main process (1094) killed by TERM signal
