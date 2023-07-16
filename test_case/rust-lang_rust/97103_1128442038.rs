plain
...i..................................................i................................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/asm_unwind_panic_abort.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/asm_unwind_panic_abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort/auxiliary"
stdout: none
--- stderr -------------------------------
error: unwinding from inline assembly is only supported on llvm >= 13.
  --> /checkout/src/test/mir-opt/asm_unwind_panic_abort.rs:12:26
   |
12 |         std::arch::asm!("", options(may_unwind));

error: aborting due to previous error
------------------------------------------

