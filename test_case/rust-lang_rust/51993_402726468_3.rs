\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs","byte_start":1058,"byte_end":1065,"line_start":37,"line_end":37,"column_start":10,"column_end":17,"is_primary":true,"text":[{"text":"    &mut 1234543 //~ ERROR","highlight_start":10,"highlight_end":17}],"label":"temporary value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs","byte_start":1076,"byte_end":1077,"line_start":38,"line_end":38,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/borrowck/promote-ref-mut-in-let-issue-46557.rs:37:10\n   |\nLL |     &mut 1234543 //~ ERROR\n   |          ^^^^^^^ temporary value does not live long enough\nLL | }\n   | - temporary value only lives until here\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:46:35] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:46:35] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:46:35] ------------------------------------------
[00:46:35] 
[00:46:35] thread '[ui] ui/borrowck/promote-ref-mut-in-let-issue-46557.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:46:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:35] 
[00:46:35] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:46:35] 
[00:46:35] 
[00:46:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:35] 
[00:46:35] 
[00:46:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:35] Build completed unsuccessfully in 0:02:11
[00:46:35] Build completed unsuccessfully in 0:02:11
[00:46:35] make: *** [check] Error 1
[00:46:35] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08d09f82
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
