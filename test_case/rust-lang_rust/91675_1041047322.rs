plain
Some tests failed in compiletest suite=codegen mode=codegen host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu

failures:

---- [codegen] codegen/sanitizer_memtag_attr_check.rs stdout ----
error: compilation failed!
error: compilation failed!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/sanitizer_memtag_attr_check.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer_memtag_attr_check" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Zsanitizer=memtag" "-Ctarget-feature=\"+mte\"" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer_memtag_attr_check/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
rustc: /checkout/src/llvm-project/llvm/lib/MC/MCSubtargetInfo.cpp:60: void ApplyFeatureFlag(llvm::FeatureBitset&, llvm::StringRef, llvm::ArrayRef<llvm::SubtargetFeatureKV>): Assertion `SubtargetFeatures::hasFlag(Feature) && "Feature flags should start with '+' or '-'"' failed.
------------------------------------------



