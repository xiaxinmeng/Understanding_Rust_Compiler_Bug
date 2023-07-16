\n\nEnsure that the expressions given can be evaluated as the33]
[00:39:33] test result: FAILED. 1304 passed; 2 failed; 4 ignored; 0 measured; 0 filtered out
[00:39:33]
[00:39:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:478:22
[00:39:33]
[00:39:33]
[00:39:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:39:33] expected success, got: exit code: 101
[00:39:33]
[00:39:33]
[00:39:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:39:33] Build completed unsuccessfully in 0:02:00
[00:39:33] make: *** [check] Error 1
[00:39:33] Makefile:58: recipe for target 'check' failed
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:00ae9360:start=1522624235534244829,finish=1522624235543013775,duration=8768946
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01eb29b4
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:01eb29b4:start=1522624235552232588,finish=1522624235560414212,duration=8181624
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0113287a
$ dmesg | grep -i kill
[   11.091276] init: failsafe main process (1092) killed by TERM signal
