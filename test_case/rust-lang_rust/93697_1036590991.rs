plain
failures:

---- [ui] ui/issues/issue-23036.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23036.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23036/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
rustc: /checkout/src/llvm-project/llvm/lib/MC/WasmObjectWriter.cpp:149: void {anonymous}::writePatchableLEB(llvm::raw_pwrite_stream&, uint64_t, uint64_t) [with int W = 5; uint64_t = long unsigned int]: Assertion `SizeLen == W' failed.
------------------------------------------



