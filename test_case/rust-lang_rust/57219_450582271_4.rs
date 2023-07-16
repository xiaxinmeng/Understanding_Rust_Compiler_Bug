\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/dropck_vec_cycle_checked.rs","byte_start":2392,"byte_end":2394,"line_start":110,"line_end":110,"column_start":25,"column_end":27,"is_primary":true,"text":[{"text":"    c3.v[1].v.set(Some(&c2));","highlight_start":25,"highlight_end":27}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/dropck_vec_cycle_checked.rs","byte_start":2444,"byte_end":2445,"line_start":112,"line_end":112,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`c2` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"values in a scope are dropped in the opposite order they are created","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: `c2` does not live long enough\n  --> /checkout/src/test/ui/span/dropck_vec_cycle_checked.rs:110:25\n   |\nLL |     c3.v[1].v.set(Some(&c2));\n   |                         ^^ borrowed value does not live long enough\nLL |     //~^ ERROR `c2` does not live long enough\nLL | }\n   | - `c2` dropped here while still borrowed\n   |\n   = note: values in a scope are dropped in the opposite order they are created\n\n"}
[00:59:58] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[00:59:58] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:59:58] ------------------------------------------
[00:59:58] 
[00:59:58] thread '[ui] ui/span/dropck_vec_cycle_checked.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:59:58] 
---
[00:59:58] 
[00:59:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:59:58] 
[00:59:58] 
[00:59:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:58] 
[00:59:58] 
[00:59:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:58] Build completed unsuccessfully in 0:03:49
[00:59:58] Build completed unsuccessfully in 0:03:49
[00:59:58] Makefile:48: recipe for target 'check' failed
[00:59:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
