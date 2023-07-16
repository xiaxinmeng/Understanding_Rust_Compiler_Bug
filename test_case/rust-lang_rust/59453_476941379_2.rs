\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/recover-from-bad-variant.rs","byte_start":179,"byte_end":188,"line_start":10,"line_end":10,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"        Enum::Foo(a, b) => {}","highlight_start":9,"highlight_end":18}],"label":"did you mean `Enum::Foo { /* fields */ }`?","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0532]: expected tuple struct/variant, found struct variant `Enum::Foo`\n  --> /checkout/src/test/ui/parser/recover-from-bad-variant.rs:10:9\n   |\nLL |         Enum::Foo(a, b) => {}\n   |         ^^^^^^^^^ did you mean `Enum::Foo { /* fields */ }`?\n\n"}
[01:13:22] {"message":"For more information about this error, try `rustc --explain E0532`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0532`.\n"}
[01:13:22] 
[01:13:22] ------------------------------------------
[01:13:22] 
---
[01:13:22] 
[01:13:22] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:22] 
[01:13:22] 
[01:13:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:22] 
[01:13:22] 
[01:13:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:23] Build completed unsuccessfully in 0:04:23
[01:13:23] Build completed unsuccessfully in 0:04:23
[01:13:23] make: *** [check] Error 1
[01:13:23] Makefile:48: recipe for target 'check' failed
52928 ./obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps
51276 ./src/llvm-project/libcxx
50800 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
50744 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib
