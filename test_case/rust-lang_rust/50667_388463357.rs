plain
[00:55:03] failures:
[00:55:03] 
[00:55:03] ---- [ui] ui-fulldeps/unnecessary-extern-crate.rs stdout ----
[00:55:03]  
[00:55:03] error: ui test compiled successfully!
[00:55:03] status: exit code: 0
[00:55:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/unnecessary-extern-crate.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/unnecessary-extern-crate.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:55:03] ------------------------------------------
[00:55:03] 
[00:55:03] ------------------------------------------
[00:55:03] stderr:
---
[00:55:03] 
[00:55:03] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:55:03] 
[00:55:03] 
[00:55:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:03] 
[00:55:03] 
[00:55:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:03] Build completed unsuccessfully in 0:16:49
[00:55:03] Build completed unsuccessfully in 0:16:49
[00:55:03] make: *** [check] Error 1
[00:55:03] Makefile:58: recipe for target 'check' failed
2939368 ./obj
2937108 ./obj/build
2162080 ./obj/build/x86_64-unknown-linux-gnu
725912 ./src
---
149744 ./.git/modules
149740 ./.git/modules/src
149124 ./src/llvm-emscripten/test
124332 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124328 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f0y00zypmb-1bxjef-22v3gsapbxtg7
116624 .:end:after_failure.2
travis_time:start:0706b7e7
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0706b7e7:start=1526066978521963326,finish=1526066978527265621,duration=5302295
