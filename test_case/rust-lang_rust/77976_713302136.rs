
failures:

---- [ui] ui/liveness/liveness-asm.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-asm.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-asm/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0472]: asm! is unsupported on this target
  --> /checkout/src/test/ui/liveness/liveness-asm.rs:12:5
   |
LL |     asm!("/*{0}*/", inout(reg) src); //~ WARN value assigned to `src` is never read
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: aborting due to 6 previous errors


------------------------------------------



failures:
    [ui] ui/liveness/liveness-asm.rs

test result: FAILED. 10397 passed; 1 failed; 564 ignored; 0 measured; 0 filtered out
