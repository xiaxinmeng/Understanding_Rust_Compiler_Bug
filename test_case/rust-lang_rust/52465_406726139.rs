plain
[00:50:59] status: exit code: 2
[00:50:59] command: "make"
[00:50:59] stdout:
[00:50:59] ------------------------------------------
[00:50:59] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small  foo.rs -C lto -O --target wasm32-unknown-unknown --cfg a
[00:50:59] Makefile:5: recipe for target 'all' failed
[00:50:59] ------------------------------------------
[00:50:59] stderr:
[00:50:59] ------------------------------------------
[00:50:59] error[E0463]: can't find crate for `std`
[00:50:59] error[E0463]: can't find crate for `std`
[00:50:59]   |
[00:50:59]   = note: the `wasm32-unknown-unknown` target may not be installed
[00:50:59] error: aborting due to previous error
[00:50:59] 
[00:50:59] For more information about this error, try `rustc --explain E0463`.
[00:50:59] For more information about this error, try `rustc --explain E0463`.
[00:50:59] make: *** [all] Error 1
[00:50:59] ------------------------------------------
[00:50:59] 
[00:50:59] 
[00:50:59] thread '[run-make] run-make/wasm-panic-small' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:59] 
[00:50:59] ---- [run-make] run-make/wasm-custom-section stdout ----
[00:50:59] 
[00:50:59] error: make failed
[00:50:59] error: make failed
[00:50:59] status: exit code: 2
[00:50:59] command: "make"
[00:50:59] stdout:
[00:50:59] ------------------------------------------
[00:50:59] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-custom-section/wasm-custom-section:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-custom-section/wasm-custom-section -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-custom-section/wasm-custom-section  foo.rs --target wasm32-unknown-unknown
[00:50:59] Makefile:5: recipe for target 'all' failed
[00:50:59] ------------------------------------------
[00:50:59] stderr:
[00:50:59] ------------------------------------------
[00:50:59] error[E0463]: can't find crate for `std`
[00:50:59] error[E0463]: can't find crate for `std`
[00:50:59]   |
[00:50:59]   = note: the `wasm32-unknown-unknown` target may not be installed
[00:50:59] error: aborting due to previous error
[00:50:59] 
[00:50:59] For more information about this error, try `rustc --explain E0463`.
[00:50:59] For more information about this error, try `rustc --explain E0463`.
[00:50:59] make: *** [all] Error 1
[00:50:59] ------------------------------------------
[00:50:59] 
[00:50:59] thread '[run-make] run-make/wasm-custom-section' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:59] 
[00:50:59] 
[00:50:59] ---- [run-make] run-make/wasm-import-module stdout ----
[00:50:59] 
[00:50:59] error: make failed
[00:50:59] status: exit code: 2
[00:50:59] command: "make"
[00:50:59] stdout:
[00:50:59] ------------------------------------------
[00:50:59] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-import-module/wasm-import-module:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-import-module/wasm-import-module -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-import-module/wasm-import-module  foo.rs --target wasm32-unknown-unknown
[00:50:59] Makefile:5: recipe for target 'all' failed
[00:50:59] ------------------------------------------
[00:50:59] stderr:
[00:50:59] ------------------------------------------
[00:50:59] error[E0463]: can't find crate for `std`
[00:50:59] error[E0463]: can't find crate for `std`
[00:50:59]   |
[00:50:59]   = note: the `wasm32-unknown-unknown` target may not be installed
[00:50:59] error: aborting due to previous error
[00:50:59] 
[00:50:59] For more information about this error, try `rustc --explain E0463`.
[00:50:59] For more information about this error, try `rustc --explain E0463`.
[00:50:59] make: *** [all] Error 1
[00:50:59] ------------------------------------------
[00:50:59] 
[00:50:59] 
[00:50:59] thread '[run-make] run-make/wasm-import-module' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:50:59] 
[00:50:59] failures:
[00:50:59]     [run-make] run-make/wasm-custom-section
[00:50:59]     [run-make] run-make/wasm-import-module
[00:50:59]     [run-make] run-make/wasm-import-module
[00:50:59]     [run-make] run-make/wasm-panic-small
[00:50:59] 
[00:50:59] test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[00:50:59] 
[00:50:59] 
[00:50:59] 
[00:50:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-make" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:59] 
[00:50:59] 
[00:50:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:50:59] Build completed unsuccessfully in 0:48:06
---
travis_time:end:1c8e9a9c:start=1532120432616565425,finish=1532120432626441103,duration=9875678
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:036fcdbf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1347c2dd
travis_time:start:1347c2dd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01005e06
$ dmesg | grep -i kill
