plain
[00:52:45] status: exit code: 2
[00:52:45] command: "make"
[00:52:45] stdout:
[00:52:45] ------------------------------------------
[00:52:45] echo 'fn main() {}' | LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-outputs/llvm-outputs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' - --out-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-outputs/llvm-outputs/random_directory_that_does_not_exist_ir/ --emit=llvm-ir
[00:52:45] Makefile:4: recipe for target 'all' failed
[00:52:45] ------------------------------------------
[00:52:45] stderr:
[00:52:45] ------------------------------------------
[00:52:45] error[E0463]: can't find crate for `std`
[00:52:45] error[E0463]: can't find crate for `std`
[00:52:45] 
[00:52:45] error: aborting due to previous error
[00:52:45] 
[00:52:45] For more information about this error, try `rustc --explain E0463`.
[00:52:45] make: *** [all] Error 1
[00:52:45] ------------------------------------------
[00:52:45] 
[00:52:45] thread '[run-make] run-make/llvm-outputs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3250:9
[00:52:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:52:45] test result: FAILED. 9 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:52:45] 
[00:52:45] 
[00:52:45] 
[00:52:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-make" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:45] 
[00:52:45] 
[00:52:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:52:45] Build completed unsuccessfully in 0:50:22
---
travis_time:end:08b17ba8:start=1543543948506579809,finish=1543543948519040680,duration=12460871
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06a76f1c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:077671ec
travis_time:start:077671ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08715ca7
$ dmesg | grep -i kill
