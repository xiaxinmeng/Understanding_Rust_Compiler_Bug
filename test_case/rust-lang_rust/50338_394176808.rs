plain
[00:55:10] status: exit code: 2
[00:55:10] command: "make"
[00:55:10] stdout:
[00:55:10] ------------------------------------------
[00:55:10] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/panic-impl-transitive/panic-impl-transitive:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/panic-impl-transitive/panic-impl-transitive -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/panic-impl-transitive/panic-impl-transitive  panic-impl-provider.rs
[00:55:10] Makefile:6: recipe for target 'all' failed
[00:55:10] ------------------------------------------
[00:55:10] stderr:
[00:55:10] ------------------------------------------
[00:55:10] error[E0463]: can't find crate for `core`
[00:55:10] error[E0463]: can't find crate for `core`
[00:55:10] 
[00:55:10] error: aborting due to previous error
[00:55:10] 
[00:55:10] For more information about this error, try `rustc --explain E0463`.
[00:55:10] make: *** [all] Error 101
[00:55:10] ------------------------------------------
[00:55:10] 
[00:55:10] 
[00:55:10] thread '[run-make] run-make/panic-impl-transitive' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:55:10] 
[00:55:10] 
[00:55:10] failures:
[00:55:10]     [run-make] run-make/panic-impl-transitive
[00:55:10]     [run-make] run-make/panic-impl-transitive
[00:55:10] 
[00:55:10] test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:55:10] 
[00:55:10] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:10] 
[00:55:10] 
[00:55:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-make" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:10] 
[00:55:10] 
[00:55:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:55:10] Build completed unsuccessfully in 0:52:19
