plain
[01:27:28] failures:
[01:27:28] 
[01:27:28] ---- [ui] ui/asm/invalid-inline-asm.rs stdout ----
[01:27:28] 
[01:27:28] error: Error: expected failure status (Some(1)) but received status None.
[01:27:28] status: signal: 6
[01:27:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/invalid-inline-asm.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm/auxiliary" "-A" "unused"
[01:27:28] ------------------------------------------
[01:27:28] 
[01:27:28] ------------------------------------------
[01:27:28] stderr:
[01:27:28] stderr:
[01:27:28] ------------------------------------------
[01:27:28] rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7828: void llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite): Assertion `(OpInfo.ConstraintType == TargetLowering::C_RegisterClass || OpInfo.ConstraintType == TargetLowering::C_Register) && "Unknown constraint type!"' failed.
[01:27:28] ------------------------------------------
[01:27:28] 
[01:27:28] thread '[ui] ui/asm/invalid-inline-asm.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:27:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:27:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:27:28] 
[01:27:28] ---- [ui] ui/asm/invalid-inline-asm-2.rs stdout ----
[01:27:28] 
[01:27:28] error: Error: expected failure status (Some(1)) but received status None.
[01:27:28] status: signal: 6
[01:27:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/invalid-inline-asm-2.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm-2/auxiliary" "-A" "unused"
[01:27:28] ------------------------------------------
[01:27:28] 
[01:27:28] ------------------------------------------
[01:27:28] stderr:
[01:27:28] stderr:
[01:27:28] ------------------------------------------
[01:27:28] rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7671: void llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite): Assertion `OpInfo.isIndirect && "Memory output must be indirect operand"' failed.
[01:27:28] ------------------------------------------
[01:27:28] 
[01:27:28] thread '[ui] ui/asm/invalid-inline-asm-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:27:28] 
---
[01:27:28] 
[01:27:28] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:27:28] 
[01:27:28] 
[01:27:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "ui" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:27:28] 
[01:27:28] 
[01:27:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[01:27:28] Build completed unsuccessfully in 1:24:10
---
travis_time:end:00efa900:start=1552148010045800738,finish=1552148010059162085,duration=13361347
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15b0c4a2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.32708.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
/home/travis/build/rust-lang/rust/obj/cores/core.32708.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc: Permission denied.
No stack.
travis_fold:start:crashlog
travis_fold:start:crashlog
obj/cores/core.32710.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc
/home/travis/build/rust-lang/rust/obj/cores/core.32710.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustc: Permission denied.
No stack.
travis_time:end:15b0c4a2:start=1552148010068364785,finish=1552148010351614701,duration=283249916
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06b85802
travis_time:start:06b85802
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1950e564
$ dmesg | grep -i kill
