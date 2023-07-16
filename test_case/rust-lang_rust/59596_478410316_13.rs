\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs","byte_start":1161,"byte_end":1230,"line_start":34,"line_end":34,"column_start":1,"column_end":70,"is_primary":true,"text":[{"text":"const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };","highlight_start":1,"highlight_end":70}],"label":"type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: it is undefined behavior to use this value\n  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:34:1\n   |\nLL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[01:16:54] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] 
---
[01:16:54] 
[01:16:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:16:54] 
[01:16:54] 
[01:16:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:54] 
[01:16:54] 
[01:16:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:54] Build completed unsuccessfully in 0:04:35
[01:16:54] Build completed unsuccessfully in 0:04:35
[01:16:54] make: *** [check] Error 1
[01:16:54] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20606a60
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  1 01:58:31 UTC 2019
