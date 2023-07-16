plain
failures:

---- [ui] ui/box/issue-95036.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/box/issue-95036.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/issue-95036" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/issue-95036/auxiliary"
stdout: none
--- stderr -------------------------------
rustc: /checkout/src/llvm-project/llvm/include/llvm/IR/Instructions.h:962: static llvm::GetElementPtrInst* llvm::GetElementPtrInst::Create(llvm::Type*, llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&, llvm::Instruction*): Assertion `cast<PointerType>(Ptr->getType()->getScalarType()) ->isOpaqueOrPointeeTypeMatches(PointeeType)' failed.



failures:
