\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/precise_pointer_size_matching.rs","byte_start":871,"byte_end":877,"line_start":27,"line_end":27,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"    match 0isize { //~ ERROR non-exhaustive patterns","highlight_start":11,"highlight_end":17}],"label":"patterns `-9223372036854775808isize..=-6isize` and `21isize..=9223372036854775807isize` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0004]: non-exhaustive patterns: `-9223372036854775808isize..=-6isize` and `21isize..=9223372036854775807isize` not covered\n  --> /checkout/src/test/ui/precise_pointer_size_matching.rs:27:11\n   |\nLL |     match 0isize { //~ ERROR non-exhaustive patterns\n   |           ^^^^^^ patterns `-9223372036854775808isize..=-6isize` and `21isize..=9223372036854775807isize` not covered\n\n"}
[00:47:34] {"message":"non-exhaustive pze..=18446744073709551615usize` not covered","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0004]: non-exhaustive patterns: `0usize` and `21usize..=18446744073709551615usize` not covered\n  --> /checkout/src/test/ui/precise_pointer_size_matching.rs:32:11\n   |\nLL |     match 0usize { //~ ERROR non-exhaustive patterns\n   |           ^^^^^^ patterns `0usize` and `21usize..=18446744073709551615usize` not covered\n\n"}
[00:47:34] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:34] {"message":"For more information about this error, try `rustc --explain E0004`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0004`.\n"}
[00:47:34] ------------------------------------------
[00:47:34] 
[00:47:34] thread '[ui] ui/precise_pointer_size_matching.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:47:34] 
---
[00:47:34] 
[00:47:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:47:34] 
[00:47:34] 
[00:47:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:34] 
[00:47:34] 
[00:47:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:34] Build completed unsuccessfully in 0:03:48
[00:47:34] Build completed unsuccessfully in 0:03:48
[00:47:34] make: *** [check] Error 1
[00:47:34] Makefile:58: recipe for target 'check' failed
