plain
---- [codegen] src/test/codegen/avr/avr-func-addrspace.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/avr/avr-func-addrspace.rs" "-Zthreads=1" "-O" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/avr/avr-func-addrspace" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "--target=avr-unknown-gnu-atmega328" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/avr/avr-func-addrspace/auxiliary" "--emit=llvm-ir"
stdout: none
--- stderr -------------------------------
error: requires `tuple_trait` lang_item
   |
   |
53 |     extern "rust-call" fn call_once(self, args: A) -> R {

error: aborting due to previous error
------------------------------------------

