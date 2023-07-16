\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/ref_to_int_match.rs","byte_start":527,"byte_end":535,"line_start":14,"line_end":14,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        10...BAR => {}, //~ ERROR lower range bound must be less than or equal to upper","highlight_start":9,"highlight_end":17}],"label":"expected u64, found u32","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/const-eval/ref_to_int_match.rs:14:9\n   |\nLL |         10...BAR => {}, //~ ERROR lower range bound must be less than or equal to upper\n   |         ^^^^^^^^ expected u64, found u32\n\n"}
[00:50:58] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:58] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] thread '[ui] ui/const-eval/ref_to_int_match.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:50:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:58] 
[00:50:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:58] 
[00:50:58] 
[00:50:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "ui" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:58] 
[00:50:58] 
[00:50:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[00:50:58] Build completed unsuccessfully in 0:48:01
