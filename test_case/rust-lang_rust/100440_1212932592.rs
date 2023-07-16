plain
---- [ui] src/test/ui/debuginfo/debuginfo-emit-llvm-ir-and-split-debuginfo.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/debuginfo/debuginfo-emit-llvm-ir-and-split-debuginfo.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-emit-llvm-ir-and-split-debuginfo" "-A" "unused" "-Crpath" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-g" "--emit=llvm-ir" "-Csplit-debuginfo=unpacked" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-emit-llvm-ir-and-split-debuginfo/auxiliary"
stdout: none
--- stderr -------------------------------
error: `-Csplit-debuginfo=unpacked` is unstable on this platform
error: aborting due to previous error
------------------------------------------


