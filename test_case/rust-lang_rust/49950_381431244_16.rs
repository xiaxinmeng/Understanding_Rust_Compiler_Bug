\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/issue-23305.rs","byte_start":530,"byte_end":534,"line_start":15,"line_end":15,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"impl ToNbt<Self> {}","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"...which again requires processing `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>`\n  --> /checkout/src/test/ui/resolve/issue-23305.rs:15:12\n   |\nLL | impl ToNbt<Self> {}\n   |            ^^^^\n   |\n   = note: ...which again requires processing `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>`, completing the cycle\n\n"}
[00:44:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:35] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
---
[00:44:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:35] expected success, got: exit code: 101
[00:44:35]
[00:44:35]
[00:44:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:35] Build completed unsuccessfully in 0:01:47
[00:44:35] Makefile:58: recipe for target 'check' failed
[00:44:35] make: *** [check] Error 1
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0a75242f:start=1523820190670923246,finish=1523820190679881645,duration=8958399
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:000f0249
$ dmesg | grep -i kill
[   11.173530] init: failsafe main process (1095) killed by TERM signal
