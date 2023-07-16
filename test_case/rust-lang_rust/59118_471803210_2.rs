\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/traits/trait-alias-syntax.rs","byte_start":648,"byte_end":657,"line_start":22,"line_end":22,"column_start":46,"column_end":55,"is_primary":true,"text":[{"text":"trait SelfWhere<One, Two> = Assoc<Two> where Self::Out: Assoc<One>;","highlight_start":46,"highlight_end":55}],"label":"associated type `Out` not found","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0220]: associated type `Out` not found for `Self`\n  --> /checkout/src/test/run-pass/traits/trait-alias-syntax.rs:22:46\n   |\nLL | trait SelfWhere<One, Two> = Assoc<Two> where Self::Out: Assoc<One>;\n   |                                              ^^^^^^^^^ associated type `Out` not found\n\n"}
[01:19:35] {"message":"For more information about this error, try `rustc --explain E0220`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0220`.\n"}
[01:19:35] 
[01:19:35] ------------------------------------------
[01:19:35] 
---
[01:19:35] 
[01:19:35] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:19:35] 
[01:19:35] 
[01:19:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:35] 
[01:19:35] 
[01:19:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:35] Build completed unsuccessfully in 0:10:58
[01:19:35] Build completed unsuccessfully in 0:10:58
[01:19:35] Makefile:48: recipe for target 'check' failed
[01:19:35] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0423c1b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 01:01:40 UTC 2019
