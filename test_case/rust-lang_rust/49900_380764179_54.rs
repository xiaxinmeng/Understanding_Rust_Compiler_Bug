\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/borrowck-object-mutability.rs","byte_start":776,"byte_end":777,"line_start":29,"line_end":29,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    x.borrowed_mut(); //~ ERROR cannot borrow","highlight_start":5,"highlight_end":6}],"label":"cannot borrow as mutable","suggested_replacement":null,"expansion":null}],"children":[{"message":"the value which is causing this path not to be mutable is...: `x`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable item `*x` as mutable\n  --> /checkout/src/test/ui/span/borrowck-object-mutability.rs:29:5\n   |\nLL |     x.borrowed_mut(); //~ ERROR cannot borrow\n   |     ^ cannot borrow as mutable\n   |\n   = note: the value which is causing this path not to be mutable is...: `x`\n\n"}
[00:49:37] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:37] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:49:37]
[00:49:37] ------------------------------------------
[00:49:37]
[00:49:37] thread '[ui (nll)] ui/span/borrowck-object-mutability.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2919:9
[00:49:37]
[00:49:37]
[00:49:37] failures:
[00:49:37]     [ui (nll)] ui/borrowck/issue-45983.rs
[00:49:37]     [ui (nll)] ui/did_you_mean/issue-35937.rs
[00:49:37]     [ui (nll)] ui/did_you_mean/issue-38147-1.rs
[00:49:37]     [ui (nll)] ui/did_you_mean/issue-38147-4.rs
[00:49:37]     [ui (nll)] ui/did_you_mean/issue-39544.rs
[00:49:37]     [ui (nll)] ui/error-codes/E0389.rs
[00:49:37]     [ui (nll)] ui/issue-36400.rs
[00:49:37]     [ui (nll)] ui/rfc-2005-default-binding-mode/enum.rs
[00:49:37]     [ui (nll)] ui/rfc-2005-default-binding-mode/explicit-mut.rs
[00:49:37]     [ui (nll)] ui/span/borrowck-call-is-borrow-issue-12224.rs
[00:49:37]     [ui (nll)] ui/span/borrowck-object-mutability.rs
[00:49:37]
[00:49:37] test result: FAILED. 1309 passed; 11 failed; 4 ignored; 0 measured; 0 filtered out
[00:49:37]
[00:49:37]
[00:49:37]
[00:49:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:49:37] expected success, got: exit code: 101
[00:49:37]
[00:49:37]
[00:49:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:37] Build completed unsuccessfully in 0:04:13
[00:49:37] Makefile:58: recipe for target 'check' failed
[00:49:37] make: *** [check] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:05bef5e6:start=1523530600526174984,finish=1523530600533075003,duration=6900019
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:03b0fe0f
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:03b0fe0f:start=1523530600541165676,finish=1523530600548258803,duration=7093127
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0852a5b4
$ dmesg | grep -i kill
[   10.504007] init: failsafe main process (1092) killed by TERM signal
