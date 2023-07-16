plain
test [ui] tests/ui/transmutability/primitives/numbers.rs ... ok

failures:

---- [ui] tests/ui/llvm-old-style-ptrs.rs stdout ----
error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/llvm-old-style-ptrs.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-old-style-ptrs/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/llvm-old-style-ptrs/auxiliary" "--target" "x86_64-unknown-linux-gnu" "-Copt-level=0" "-Cllvm-args=-opaque-pointers=0"
stdout: none
--- stderr -------------------------------
rustc: /checkout/src/llvm-project/llvm/lib/IR/Instructions.cpp:1477: llvm::LoadInst::LoadInst(llvm::Type *, llvm::Value *, const llvm::Twine &, bool, llvm::Align, llvm::AtomicOrdering, SyncScope::ID, llvm::Instruction *): Assertion `cast<PointerType>(Ptr->getType())->isOpaqueOrPointeeTypeMatches(Ty)' failed.



failures:
