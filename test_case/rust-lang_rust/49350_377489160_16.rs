\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gate-macros_in_extern.rs","byte_start":1018,"byte_end":1035,"line_start":33,"line_end":33,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    emits_nothing!();","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"add #![feature(macros_in_extern)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: Macro invocations in `extern {}` blocks are experimental. (see issue #49476)\n  --> /checkout/src/test/ui/feature-gate-macros_in_extern.rs:33:5\n   |\nLL |     emits_nothing!();\n   |     ^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(macros_in_extern)] to the crate attributes to enable\n\n"}
[00:36:07] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:36:07] {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
---
[00:36:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:36:07] expected success, got: exit code: 101
[00:36:07]
[00:36:07]
[00:36:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:36:07] Build completed unsuccessfully in 0:01:49
[00:36:07] make: *** [check] Error 1
[00:36:07] Mak3545748 .
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs
