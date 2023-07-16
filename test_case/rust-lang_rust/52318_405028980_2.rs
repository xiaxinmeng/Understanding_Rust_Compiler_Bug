\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/closure_promotion.rs","byte_start":526,"byte_end":545,"line_start":14,"line_end":14,"column_start":26,"column_end":45,"is_primary":true,"text":[{"text":"    let x: &'static _ = &|| { let z = 3; z }; //~ ERROR does not live long enough","highlight_start":26,"highlight_end":45}],"label":"temporary value does not live long enough","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/const-eval/closure_promotion.rs","byte_start":583,"byte_end":584,"line_start":15,"line_end":15,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"temporary value only lives until here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the static lifetime...","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0597]: borrowed value does not live long enough\n  --> /checkout/src/test/ui/const-eval/closure_promotion.rs:14:26\n   |\nLL |     let x: &'static _ = &|| { let z = 3; z }; //~ ERROR does not live long enough\n   |                          ^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough\nLL | }\n   | - temporary value only lives until here\n   |\n   = note: borrowed value must be valid for the static lifetime...\n\n"}
[00:48:40] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:40] {"message":"For more information about this error, try `rustc --explain E0597`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0597`.\n"}
[00:48:40] ------------------------------------------
[00:48:40] 
[00:48:40] thread '[ui] ui/const-eval/closure_promotion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:48:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:40] 
[00:48:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:48:40] 
[00:48:40] 
[00:48:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:40] 
[00:48:40] 
[00:48:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:40] Build completed unsuccessfully in 0:01:29
[00:48:40] Build completed unsuccessfully in 0:01:29
[00:48:40] Makefile:58: recipe for target 'check' failed
[00:48:40] make: *** [check] Error 1
125372 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
122684 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
107620 ./src/llvm/test/CodeGen
107600 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
