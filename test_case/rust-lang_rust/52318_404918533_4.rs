\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs","byte_start":719,"byte_end":727,"line_start":21,"line_end":21,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    v2.push(&young[0]);      // statement 4","highlight_start":14,"highlight_end":22}],"label":"borrowed value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs","byte_start":2145,"byte_end":2146,"line_start":58,"line_end":58,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`young[..]` dropped here while still borrowed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"values in a scope are dropped in the opposite order they are created","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: `young[..]` does not live long enough\n  --> /checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs:21:14\n   |\nLL |     v2.push(&young[0]);      // statement 4\n   |              ^^^^^^^^ borrowed value does not live long enough\n...\nLL | }\n   | - `young[..]` dropped here while still borrowed\n   |\n   = note: values in a scope are dropped in the opposite order they are created\n\n"}
[00:45:13] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:13] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] thread '[ui] ui/span/borrowck-let-suggestion-suffixes.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:45:13] 
[00:45:13] 
[00:45:13] ---- [ui] ui/span/issue-15480.rs stdout ----
[00:45:13] 
[00:45:13] error: ui test compiled successfully!
[00:45:13] status: exit code: 0
[00:45:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-15480.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-15480/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-15480/auxiliary" "-A" "unused"
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] ------------------------------------------
[00:45:13] stderr:
---
[00:45:13] ---- [ui] ui/span/regions-close-over-borrowed-ref-in-obj.rs stdout ----
[00:45:13] 
[00:45:13] error: ui test compiled successfully!
[00:45:13] status: exit code: 0
[00:45:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/regions-close-over-borrowed-ref-in-obj.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-close-over-borrowed-ref-in-obj/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-close-over-borrowed-ref-in-obj/auxiliary" "-A" "unused"
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] ------------------------------------------
[00:45:13] stderr:
---
[00:45:13] ---- [ui] ui/suggestions/issue-52049.rs stdout ----
[00:45:13] 
[00:45:13] error: ui test compiled successfully!
[00:45:13] status: exit code: 0
[00:45:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-52049.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-52049/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-52049/auxiliary" "-A" "unused"
[00:45:13] ------------------------------------------
[00:45:13] 
[00:45:13] ------------------------------------------
[00:45:13] stderr:
---
[00:45:13] test result: FAILED. 1551 passed; 6 failed; 5 ignored; 0 measured; 0 filtered out
[00:45:13] 
[00:45:13] 
[00:45:13] 
[00:45:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:45:13] 
[00:45:13] 
[00:45:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:13] Build completed unsuccessfully in 0:01:25
[00:45:13] Build completed unsuccessfully in 0:01:25
[00:45:13] Makefile:58: recipe for target 'check' failed
[00:45:13] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05c50b81
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
