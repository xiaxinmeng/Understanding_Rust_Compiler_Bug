\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs","byte_start":177,"byte_end":184,"line_start":12,"line_end":12,"column_start":20,"column_end":27,"is_primary":true,"text":[{"text":"    let x : char = last(y);","highlight_start":20,"highlight_end":27}],"label":"expected char, found enum `std::option::Option`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `char`\n   found type `std::option::Option<_>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/tag-that-dare-not-speak-its-name.rs:12:20\n   |\nLL |     let x : char = last(y);\n   |                    ^^^^^^^ expected char, found enum `std::option::Option`\n   |\n   = note: expected type `char`\n              found type `std::option::Option<_>`\n\n"}
[01:07:10] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[01:07:10] 
[01:07:10] ------------------------------------------
[01:07:10] 
---
[01:07:10] 
[01:07:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:07:10] 
[01:07:10] 
[01:07:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:10] 
[01:07:10] 
[01:07:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:10] Build completed unsuccessfully in 0:04:18
[01:07:10] Build completed unsuccessfully in 0:04:18
[01:07:10] make: *** [check] Error 1
[01:07:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:007f42b6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 15 12:40:59 UTC 2019
