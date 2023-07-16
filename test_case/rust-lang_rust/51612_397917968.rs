plain
[00:51:24] ...i..............................................................................i.................
[00:51:30] ..................................................................................................FF
[00:51:35] ....................................................................................................
[00:51:41] ....................................................................................................
,"spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":828,"byte_end":835,"line_start":25,"line_end":25,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot mutate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable item `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:25:13\n   |\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot mutate\n\n"}
[00:51:47] {"message":"cannot assign to immutable item `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":997,"byte_end":1004,"line_start":33,"line_end":33,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot mutate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable item `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:33:13\n   |\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot mutate\n\n"}
[00:51:47] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:51:47] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:51:47] ------------------------------------------
[00:51:47] 
[00:51:47] thread '[ui (nll)] ui/rfc-2005-default-binding-mode/explicit-mut.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:51:47] 
---
[00:51:47] 
[00:51:47] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:47] 
[00:51:47] 
[00:51:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:51:47] 
[00:51:47] 
[00:51:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:47] Build completed unsuccessfully in 0:03:37
[00:51:47] Build completed unsuccessfully in 0:03:37
[00:51:47] make: *** [check] Error 1
[00:51:47] Makefile:58: recipe for target 'check' failed
