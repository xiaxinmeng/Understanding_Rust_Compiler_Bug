plain
[01:35:28] ---- [run-pass] run-pass/issue-33264.rs stdout ----
[01:35:28] 
[01:35:28] error: compilation failed!
[01:35:28] status: exit code: 1
[01:35:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-33264.rs" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33264/a.js" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-33264/auxiliary"
[01:35:28] ------------------------------------------
[01:35:28] 
[01:35:28] ------------------------------------------
[01:35:28] stderr:
[01:35:28] stderr:
[01:35:28] ------------------------------------------
[01:35:28] error[E0472]: asm! is unsupported on this target
[01:35:28]    |
[01:35:28] 21 | /             asm!("
[01:35:28] 21 | /             asm!("
[01:35:28] 22 | |                  movaps $1, %xmm1
[01:35:28] 23 | |                  movaps $2, %xmm2
[01:35:28] 24 | |                  addps %xmm1, %xmm2
[01:35:28] ...  |
[01:35:28] 29 | |                  : "xmm1", "xmm2"
[01:35:28] 30 | |                  );
[01:35:28] 
[01:35:28] error: aborting due to previous error
[01:35:28] 
[01:35:28] For more information about this error, try `rustc --explain E0472`.
---
[01:35:28] 
[01:35:28] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:35:28] 
[01:35:28] 
[01:35:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/4.1.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:35:28] 
[01:35:28] 
[01:35:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
[01:35:28] Build completed unsuccessfully in 1:31:28
---
travis_time:end:22f5d3b4:start=1532278014631037651,finish=1532278014639822525,duration=8784874
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a7ee088
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1913a030
travis_time:start:1913a030
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33d0c9b0
$ dmesg | grep -i kill
