plain
[00:56:16] status: exit code: 2
[00:56:16] command: "make"
[00:56:16] stdout:
[00:56:16] ------------------------------------------
[00:56:16] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-51947/issue-51947:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-51947/issue-51947 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-51947/issue-51947  test.rs --crate-type rlib -O
[00:56:16] Makefile:4: recipe for target 'all' failed
[00:56:16] ------------------------------------------
[00:56:16] stderr:
[00:56:16] ------------------------------------------
[00:56:16] error[E0463]: can't find crate for `std`
[00:56:16] error[E0463]: can't find crate for `std`
[00:56:16] 
[00:56:16] error: aborting due to previous error
[00:56:16] 
[00:56:16] For more information about this error, try `rustc --explain E0463`.
[00:56:16] make: *** [all] Error 1
[00:56:16] ------------------------------------------
[00:56:16] 
[00:56:16] thread '[run-make] run-make/issue-51947' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:56:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:16] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:16] 
[00:56:16] 
[00:56:16] 
[00:56:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "run-make" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:16] 
[00:56:16] 
[00:56:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[00:56:16] Build completed unsuccessfully in 0:52:54
---
travis_time:end:00fce28c:start=1541509576247418052,finish=1541509576254868668,duration=7450616
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b26f07a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:062645ac
travis_time:start:062645ac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00ec8c20
$ dmesg | grep -i kill
