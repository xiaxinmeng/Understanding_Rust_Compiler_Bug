plain
Resolving deltas: 100% (611543/611543), completed with 4858 local objects.
---
[00:01:07] configure: rust.quiet-tests     := True
---
[00:53:10] ..........................................................................i.........................
[00:53:18] .................i..................................................................................
---
[00:53:59] .............................................................................................i......
[00:54:08] ...................................................................i................................
[00:54:15] ...........................................................................F........................
---
[00:54:35] error: /checkout/src/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.rs:30: unexpected error: '30:9: 30:12: cannot borrow `foo` as mutable because it is also borrowed as immutable [E0502]'
[00:54:35]
[00:54:35] error: 1 unexpected errors found, 0 expected errors not found
[00:54:35] status: exit code: 101
[00:54:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/borrowck-issue-49631.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:54:35] unexpected errors (from JSON output): [
[00:54:35]     Error {
[00:54:35]         line_num: 30,
[00:54:35]         kind: Some(
[00:54:35]             Error
[00:54:35]         ),
[00:54:35]         msg: "30:9: 30:12: cannot borrow `foo` as mutable because it is also borrowed as immutable [E0502]"
---
[00:54:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:35] expected success, got: exit code: 101
[00:54:35]
[00:54:35]
[00:54:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:35] Build completed unsuccessfully in 0:02:55
[00:54:35] make: *** [check] Error 1
[00:54:35] Makefile:58: recipe for target 'check' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0f635cd4:start=1522967267074479145,finish=1522967267080542384,duration=6063239
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04077498
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:04077498:start=1522967267085890330,finish=1522967267092082829,duration=6192499
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09afb285
$ dmesg | grep -i kill
[   10.830657] init: failsafe main process (1094) killed by TERM signal
