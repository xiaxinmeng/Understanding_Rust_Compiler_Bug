\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-48636.rs","byte_start":63,"byte_end":92,"line_start":7,"line_end":7,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    /// The ID of the parent core","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"doc comments must come before what they document, maybe a comment was intended with `//`?","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"missing comma here","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-48636.rs","byte_start":58,"byte_end":58,"line_start":6,"line_end":6,"column_start":10,"column_end":10,"is_primary":true,"text":[{"text":"    x: u8","highlight_start":10,"highlight_end":10}],"label":null,"suggested_replacement":",","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0585]: found a documentation comment that doesn't document anything\n  --> /checkout/src/test/ui/issues/issue-48636.rs:7:5\n   |\nLL |     x: u8\n   |          - help: missing comma here: `,`\nLL |     /// The ID of the parent core\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: doc comments must come before what they document, maybe a comment was intended with `//`?\n\n"}
[01:01:21] {"message":"For more information about this error, try `rustc --explain E0585`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0585`.\n"}
[01:01:21] 
[01:01:21] ------------------------------------------
[01:01:21] 
---
[01:01:21] 
[01:01:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:01:21] 
[01:01:21] 
[01:01:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:21] 
[01:01:21] 
[01:01:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:21] Build completed unsuccessfully in 0:04:17
[01:01:21] Build completed unsuccessfully in 0:04:17
[01:01:21] Makefile:48: recipe for target 'check' failed
[01:01:21] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0024e488
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 18:49:05 UTC 2019
