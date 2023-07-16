plain
[01:02:07] failures:
[01:02:07] 
[01:02:07] ---- [ui] ui/asm/invalid-inline-asm-2.rs stdout ----
[01:02:07] 
[01:02:07] error: Error: expected failure status (Some(1)) but received status None.
[01:02:07] status: signal: 6
[01:02:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/invalid-inline-asm-2.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm-2/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm-2/auxiliary" "-A" "unused"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7669: void llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite): Assertion `OpInfo.isIndirect && "Memory output must be indirect operand"' failed.
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] thread '[ui] ui/asm/invalid-inline-asm-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:02:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:02:07] 
[01:02:07] ---- [ui] ui/asm/invalid-inline-asm.rs stdout ----
[01:02:07] 
[01:02:07] error: Error: expected failure status (Some(1)) but received status None.
[01:02:07] status: signal: 6
[01:02:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/invalid-inline-asm.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm/auxiliary" "-A" "unused"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
[01:02:07] stderr:
[01:02:07] ------------------------------------------
[01:02:07] rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7826: void llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite): Assertion `(OpInfo.ConstraintType == TargetLowering::C_RegisterClass || OpInfo.ConstraintType == TargetLowering::C_Register) && "Unknown constraint type!"' failed.
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] thread '[ui] ui/asm/invalid-inline-asm.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:02:07] 
[01:02:07] 
[01:02:07] ---- [ui] ui/malformed/malformed-unwind-2.rs stdout ----
[01:02:07] 
[01:02:07] error: ui test compiled successfully!
[01:02:07] status: exit code: 0
[01:02:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/malformed/malformed-unwind-2.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-unwind-2/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-unwind-2/auxiliary" "-A" "unused"
[01:02:07] ------------------------------------------
[01:02:07] 
[01:02:07] ------------------------------------------
[01:02:07] stderr:
---
[01:02:07] 
[01:02:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:02:07] 
[01:02:07] 
[01:02:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:07] 
[01:02:07] 
[01:02:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:02:07] Build completed unsuccessfully in 0:58:56
---
travis_time:end:0612bb5c:start=1551230448772941934,finish=1551230448795790706,duration=22848772
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a0c5336
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.406.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 430]
[New LWP 418]
[New LWP 406]
[New LWP 429]
[New LWP 409]
[New LWP 417]
warning: Could not load shared library symbols for 8 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007fcba9c5fe97 in ?? ()
[Current thread is 1 (LWP 430)]
#0  0x00007fcba9c5fe97 in ?? ()
#1  0x0000000000000000 in ?? ()
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.408.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
[New LWP 433]
[New LWP 432]
[New LWP 408]
[New LWP 412]
[New LWP 423]
[New LWP 424]
warning: Could not load shared library symbols for 8 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/tes'.
Program terminated with signal SIGABRT, Aborted.
#0  0x00007fe2e42cde97 in ?? ()
[Current thread is 1 (LWP 433)]
#0  0x00007fe2e42cde97 in ?? ()
#1  0x0000000000000000 in ?? ()
travis_time:end:0a0c5336:start=1551230448811613347,finish=1551230452457943389,duration=3646330042
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08cbcfb3
travis_time:start:08cbcfb3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a8b9b10
$ dmesg | grep -i kill
