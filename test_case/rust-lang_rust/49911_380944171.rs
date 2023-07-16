plain
[00:00:44] configure: rust.quiet-tests     := True
---
[00:56:25] ...............................................................................i....................
[00:56:32] ......................i.............................................................................
---
[00:57:23] ..i...........................................................................i.....................
---
[00:58:37] .............................................i......................................................
---
[01:03:42] .............................i......................................................................
[01:03:59] ..............................................................i.....................................
[01:04:19] ................................................i...................................................
[01:04:44] ....................................................................................................
[01:05:11] ....................................................................................................
[01:05:38] ....................................................................................................
[01:06:09] ......i.............................................................................................
[01:06:29] ..i...........................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[01:06:55] ......................................................
[01:07:35] ....................................................................................................
[01:08:18] ....................................................F...............ii..............................
[01:09:03] ...............................i.........................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:09:24] ...........................i.ii............
[01:10:15] ............................................................................................iiiiiii.
---
[01:13:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rfc-1937-termination-trait/termination-trait-in-test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfc-1937-termination-trait/termination-trait-in-test.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfc-1937-termination-trait/termination-trait-in-test.stage2-x86_64-unknown-linux-gnu.aux"
---
[01:13:13] error: functions used as tests returning Result<_, _> must not use #[should_panic]
[01:13:13]   --> /checkout/src/test/run-pass/rfc-1937-termination-trait/termination-trait-in-test.rs:28:1
[01:13:13]    |
[01:13:13] 28 | / fn not_a_num() -> Result<(), ParseIntError> {
[01:13:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:486:22
[01:13:13] 29 | |     //~^ ERROR test functions returning Result<_, _> must not use #[should_panic]
[01:13:13] 30 | |     let _: u32 = "abc".parse()?;
[01:13:13] 31 | |     Ok(())
---
[01:13:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:13] expected success, got: exit code: 101
[01:13:13]
[01:13:13]
[01:13:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:13] Build completed unsuccessfully in 0:18:22
[01:13:13] Makefile:58: recipe for target 'check' failed
[01:13:13] make: *** [check] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:02b1e9e6:start=1523567133345765549,finish=1523567133351418619,duration=5653070
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1b3d3a5c
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1b3d3a5c:start=1523567133356503736,finish=1523567133361696381,duration=5192645
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:073f3a70
$ dmesg | grep -i kill
[   10.488973] init: failsafe main process (1093) killed by TERM signal
