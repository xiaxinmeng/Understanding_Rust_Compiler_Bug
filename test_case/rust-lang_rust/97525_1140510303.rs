plain
---- [ui] src/test/ui/asm/issue-97490.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-97490.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-97490" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-97490/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: inline assembly is not stable yet on this architecture
   |
   |
LL |         core::arch::asm!("call {}", in(reg) yes, options(noreturn));
   |
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = note: see issue #93335 <https://github.com/rust-lang/rust/issues/93335> for more information
   = help: add `#![feature(asm_experimental_arch)]` to the crate attributes to enable

error: invalid register class `reg`: unknown register class
   |
   |
LL |         core::arch::asm!("call {}", in(reg) yes, options(noreturn));

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
