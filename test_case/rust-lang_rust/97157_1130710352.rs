plain
test [mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/asm_unwind_panic_abort.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/asm_unwind_panic_abort.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/asm_unwind_panic_abort/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
  --> /checkout/src/test/mir-opt/asm_unwind_panic_abort.rs:13:9
   |
13 |         std::arch::asm!("", options(may_unwind));
   |
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
