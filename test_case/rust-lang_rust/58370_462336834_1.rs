\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-35677.rs","byte_start":106,"byte_end":115,"line_start":4,"line_end":4,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"    this.is_subset(other)","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the method `is_subset` exists but the following trait bounds were not satisfied:\n`T : std::cmp::Eq`\n`T : std::hash::Hash`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `is_subset` found for type `&std::collections::HashSet<T>` in the current scope\n  --> /checkout/src/test/ui/issues/issue-35677.rs:4:10\n   |\nLL |     this.is_subset(other)\n   |          ^^^^^^^^^\n   |\n   = note: the method `is_subset` exists but the following trait bounds were not satisfied:\n           `T : std::cmp::Eq`\n           `T : std::hash::Hash`\n\n"}
[01:05:36] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[01:05:36] 
[01:05:36] ------------------------------------------
[01:05:36] 
---
[01:05:36] 
[01:05:36] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:05:36] 
[01:05:36] 
[01:05:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:36] 
[01:05:36] 
[01:05:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:36] Build completed unsuccessfully in 0:04:38
[01:05:36] Build completed unsuccessfully in 0:04:38
[01:05:36] Makefile:48: recipe for target 'check' failed
[01:05:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23a82bd0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 13:59:44 UTC 2019
