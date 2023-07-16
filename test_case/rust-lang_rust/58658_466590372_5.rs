\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/ub-ref.rs","byte_start":717,"byte_end":788,"line_start":20,"line_end":20,"column_start":1,"column_end":72,"is_primary":true,"text":[{"text":"const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };","highlight_start":1,"highlight_end":72}],"label":"type validation failed: encountered integer pointer in non-ZST reference","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: it is undefined behavior to use this value\n  --> /checkout/src/test/ui/consts/const-eval/ub-ref.rs:20:1\n   |\nLL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered integer pointer in non-ZST reference\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[01:06:26] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
[01:06:26] 
[01:06:26] ------------------------------------------
[01:06:26] 
---
[01:06:26] 
[01:06:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:06:26] 
[01:06:26] 
[01:06:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:26] 
[01:06:26] 
[01:06:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:26] Build completed unsuccessfully in 0:04:33
[01:06:26] Build completed unsuccessfully in 0:04:33
[01:06:26] make: *** [check] Error 1
[01:06:26] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c618f30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb 23 00:04:28 UTC 2019
---
travis_time:end:15c0a9de:start=1550880269840333365,finish=1550880269848376739,duration=8043374
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:019b25e3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04acc019
$ dmesg | grep -i kill
