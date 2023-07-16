
[01:04:03] ---- [ui] ui/asm/invalid-inline-asm-2.rs stdout ----
[01:04:03] 
[01:04:03] error: Error: expected failure status (Some(1)) but received status None.
[01:04:03] status: signal: 6
[01:04:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/invalid-inline-asm-2.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm-2/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm-2/auxiliary" "-A" "unused"
[01:04:03] stdout:
[01:04:03] ------------------------------------------
[01:04:03] 
[01:04:03] ------------------------------------------
[01:04:03] stderr:
[01:04:03] ------------------------------------------
[01:04:03] rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7671: void llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite): Assertion `OpInfo.isIndirect && "Memory output must be indirect operand"' failed.
[01:04:03] 
[01:04:03] ------------------------------------------
[01:04:03] 
[01:04:03] thread '[ui] ui/asm/invalid-inline-asm-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:04:03] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:03] 
[01:04:03] ---- [ui] ui/asm/invalid-inline-asm.rs stdout ----
[01:04:03] 
[01:04:03] error: Error: expected failure status (Some(1)) but received status None.
[01:04:03] status: signal: 6
[01:04:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/invalid-inline-asm.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/invalid-inline-asm/auxiliary" "-A" "unused"
[01:04:03] stdout:
[01:04:03] ------------------------------------------
[01:04:03] 
[01:04:03] ------------------------------------------
[01:04:03] stderr:
[01:04:03] ------------------------------------------
[01:04:03] rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7828: void llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite): Assertion `(OpInfo.ConstraintType == TargetLowering::C_RegisterClass || OpInfo.ConstraintType == TargetLowering::C_Register) && "Unknown constraint type!"' failed.
[01:04:03] 
[01:04:03] ------------------------------------------
[01:04:03] 
