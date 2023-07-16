plain
Resolving deltas: 100% (614190/614190), completed with 4873 local objects.
---
[00:01:10] configure: rust.quiet-tests     := True
---
[00:39:03] ...............................................................................i....................
[00:39:09] ......................i.............................................................................
---
[00:39:47] ..................i.............................................F.............................i.....
[00:39:53] ....................................................................................................
[00:39:59] ........ii..........................................................................................
t":5,"highlight_end":22}],"label":"`fancy_ref` is a `&` reference, so the data it refers to cannot be written","suggested_replacement":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference: `&mut (&mut fancy)`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-47388.rs","byte_start":589,"byte_end":602,"line_start":17,"line_end":17,"column_start":21,"column_end":34,"is_primary":true,"text":[{"text":"    let fancy_ref = &(&mut fancy);","highlight_start":21,"highlight_end":34}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0594]: cannot assign to data in a `&` reference\n  --> /checkout/src/test/ui/nll/issue-47388.rs:18:5\n   |\nLL |     fancy_ref.num = 6; //~ ERROR E0594\n   |     ^^^^^^^^^^^^^^^^^ `fancy_ref` is a `&` reference, so the data it refers to cannot be written\nhelp: consider changing this to be a mutable reference: `&mut (&mut fancy)`\n   |\nLL |     let fancy_ref = ;\n   | \n\n"}
[00:40:10] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:40:10] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
---
[00:40:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:40:10] expected success, got: exit code: 101
[00:40:10]
[00:40:10]
[00:40:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:40:10] Build completed unsuccessfully in 0:02:10
[00:40:10] make: *** [check] Error 1
[00:40:10] Makefile:58: recipe for target 'check' failed
---
121748 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv/s-f02y1pcv77-9ggsry-3ijxj2oqwtm6x
109928 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
109924 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
106044 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102808 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102612 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
102608 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f02yz8ngw0-uorcok-1wdf9ktk9nlt9
100736 ./obj/build/bootstrap/debug/incremental/bootstrap-zki88qpb4pjm
100732 ./obj/build/bootstrap/debug/incremental/bootstrap-zki88qpb4pjm/s-f02z1dktqh-154ep14-1fnjifxdmfg4c
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:080d49bf:start=1523632468355886030,finish=1523632468362280230,duration=6394200
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:06ccb565
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:06ccb565:start=1523632468367257706,finish=1523632468372424697,duration=5166991
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:054a132c
$ dmesg | grep -i kill
[    9.787893] init: failsafe main process (1095) killed by TERM signal
