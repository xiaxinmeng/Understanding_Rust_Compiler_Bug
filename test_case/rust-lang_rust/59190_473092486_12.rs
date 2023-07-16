\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-22560.rs","byte_start":66,"byte_end":145,"line_start":5,"line_end":8,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"type Test = Add +","highlight_start":13,"highlight_end":18},{"text":"            //~^ ERROR E0393","highlight_start":1,"highlight_end":29},{"text":"            //~| ERROR E0191","highlight_start":1,"highlight_end":29},{"text":"            Sub;","highlight_start":1,"highlight_end":16}],"label":"associated type `Output` must be specified","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0191]: the value of the associated type `Output` (from the trait `std::ops::Add`) must be specified\n  --> /checkout/src/test/ui/issues/issue-22560.rs:5:13\n   |\nLL |   type Test = Add +\n   |  _____________^\nLL | |             //~^ ERROR E0393\nLL | |             //~| ERROR E0191\nLL | |             Sub;\n   | |_______________^ associated type `Output` must be specified\n\n"}
[01:15:17] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:15:17] {"message":"Some errors occurred: E0191, E0225, E0393.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0225, E0393.\n"}
[01:15:17] 
[01:15:17] ------------------------------------------
[01:15:17] 
[01:15:17] thread '[ui] ui/issues/issue-22560.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:15:17] 
[01:15:17] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:15:17] 
[01:15:17] 
[01:15:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:17] 
[01:15:17] 
[01:15:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:17] Build completed unsuccessfully in 0:04:20
