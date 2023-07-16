plain
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....................................i..ii.i.ii................
failures:

---- [assembly] src/test/assembly/asm/aarch64-el2vmsa.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/asm/aarch64-el2vmsa.rs" "-Zthreads=1" "-O" "--emit" "asm" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-el2vmsa/aarch64-el2vmsa.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/asm/aarch64-el2vmsa/auxiliary" "--target" "aarch64-unknown-linux-gnu"
stdout: none
--- stderr -------------------------------
  |
  = note: the `aarch64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add aarch64-unknown-linux-gnu`
  = help: consider downloading the target with `rustup target add aarch64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error[E0463]: can't find crate for `compiler_builtins`

error: cannot determine resolution for the macro `asm`
error: cannot determine resolution for the macro `asm`
  --> /checkout/src/test/assembly/asm/aarch64-el2vmsa.rs:28:9
   |
28 |         asm!("msr vttbr_el2, x0");
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the macro `asm`
error: cannot determine resolution for the macro `asm`
  --> /checkout/src/test/assembly/asm/aarch64-el2vmsa.rs:17:9
   |
17 |         asm!("msr ttbr0_el2, x0");
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: requires `sized` lang_item

error: aborting due to 5 previous errors


For more information about this error, try `rustc --explain E0463`.
------------------------------------------



failures:
    [assembly] src/test/assembly/asm/aarch64-el2vmsa.rs
test result: FAILED. 127 passed; 1 failed; 22 ignored; 0 measured; 0 filtered out; finished in 0.51s

Build completed unsuccessfully in 0:11:19
