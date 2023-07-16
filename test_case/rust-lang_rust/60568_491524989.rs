plain
[01:09:37] status: exit code: 2
[01:09:37] command: "make"
[01:09:37] stdout:
[01:09:37] ------------------------------------------
[01:09:37] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small  foo.rs -C lto -O --target wasm32-unknown-unknown --cfg a
[01:09:37] wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm
[01:09:37] 50740
[01:09:37] [ "`wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-panic-small/wasm-panic-small/foo.wasm`" -lt "1024" ]
[01:09:37] Makefile:6: recipe for target 'all' failed
[01:09:37] ------------------------------------------
[01:09:37] stderr:
[01:09:37] ------------------------------------------
[01:09:37] make: *** [all] Error 1
---
[01:09:37] status: exit code: 2
[01:09:37] command: "make"
[01:09:37] stdout:
[01:09:37] ------------------------------------------
[01:09:37] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-stringify-ints-small/wasm-stringify-ints-small:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-stringify-ints-small/wasm-stringify-ints-small -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-stringify-ints-small/wasm-stringify-ints-small  foo.rs -C lto -O --target wasm32-unknown-unknown
[01:09:37] wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-stringify-ints-small/wasm-stringify-ints-small/foo.wasm
[01:09:37] 189258
[01:09:37] [ "`wc -c < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-stringify-ints-small/wasm-stringify-ints-small/foo.wasm`" -lt "20500" ]
[01:09:37] Makefile:5: recipe for target 'all' failed
[01:09:37] ------------------------------------------
[01:09:37] stderr:
[01:09:37] ------------------------------------------
[01:09:37] make: *** [all] Error 1
---
[01:09:37] test result: FAILED. 8 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out
[01:09:37] 
[01:09:37] 
[01:09:37] 
[01:09:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-make" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:37] 
[01:09:37] 
[01:09:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:09:37] Build completed unsuccessfully in 1:00:45
---
travis_time:end:02897a22:start=1557591850229103594,finish=1557591850237500101,duration=8396507
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:038b514d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0494db58
travis_time:start:0494db58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27ab0bc4
$ dmesg | grep -i kill
