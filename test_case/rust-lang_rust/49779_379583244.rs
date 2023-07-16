plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:41:48] ............................................................................i.......................
[00:41:54] .....F..F............i..............................................................................
---
[00:42:30] ...................................................................................................i
[00:42:37] .........................................................................i..........................
[00:42:42] ....................................................................................................
[00:42:49] ....................................................................................................
[00:42:57] ....................................................................................................
ant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/issue-43197.rs","byte_start":715,"byte_end":716,"line_start":22,"line_end":22,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"    println!(\"{} {}\", X, Y);","highlight_start":26,"highlight_end":27}],"label":"referenced constant has errors","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/issue-43197.rs:22:26\n   |\nLL |     println!(\"{} {}\", X, Y);\n   |                          ^ referenced constant has errors\n\n"}
---
[00:42:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:59] expected success, got: exit code: 101
[00:42:59]
[00:42:59]
[00:42:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:59] Build completed unsuccessfully in 0:02:21
[00:42:59] make: *** [check] Error 1
[00:42:59] Makefile:58: recipe for target 'check' failed
