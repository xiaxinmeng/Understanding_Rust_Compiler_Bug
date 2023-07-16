\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":687,"byte_end":688,"line_start":23,"line_end":23,"column_start":12,"column_end":13,"is_primary":false,"text":[{"text":"    fn bar<U: Debug>(&self, _: &U);","highlight_start":12,"highlight_end":13}],"label":"declaration in trait here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs","byte_start":755,"byte_end":765,"line_start":27,"line_end":27,"column_start":23,"column_end":33,"is_primary":true,"text":[{"text":"    fn bar(&self, _: &impl De                         ^^^^^^^^^^^ expected generic parameter, found `impl Trait`\n   | \n  ::: /checkout/src/libcore/hash/mod.rs:185:13\n   |\nLL |     fn hash<H: Hasher>(&self, state: &mut H);\n   |             - declaration in trait here\n\n"}
[00:43:56] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:43:56] {"message":"For more information about this error, try `rustc --explain E0643`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0643`.\n"}
[00:43:56] ------------------------------------------
[00:43:56] 
[00:43:56] thread '[ui] ui/impl-trait/impl-generic-mismatch.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:43:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:43:56] 
[00:43:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:56] 
[00:43:56] 
[00:43:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:43:56] 
[00:43:56] 
[00:43:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:56] Build completed unsuccessfully in 0:02:26
[00:43:56] Build completed unsuccessfully in 0:02:26
[00:43:56] Makefile:58: recipe for target 'check' failed
[00:43:56] make: *** [check] Error 1
61608 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
60840 ./src/llvm-emscripten/lib
59864 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
56340 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
