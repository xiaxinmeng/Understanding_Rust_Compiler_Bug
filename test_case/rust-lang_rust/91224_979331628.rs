plain
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#2d4fea73)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0004]: non-exhaustive patterns: `Avr(_)` not covered
    |
    |
558 |         InlineAsmRegOrRegClass::RegClass(reg) => match reg {
    |                                                        ^^^ pattern `Avr(_)` not covered
   ::: /checkout/compiler/rustc_target/src/asm/mod.rs:407:5
    |
    |
407 |     Avr(AvrInlineAsmRegClass),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`

error[E0004]: non-exhaustive patterns: `Avr(_)` not covered
    |
616 |     match reg {
616 |     match reg {
    |           ^^^ pattern `Avr(_)` not covered
   ::: /checkout/compiler/rustc_target/src/asm/mod.rs:407:5
    |
    |
407 |     Avr(AvrInlineAsmRegClass),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`

error[E0004]: non-exhaustive patterns: `Avr(_)` not covered
    |
724 |     match reg {
724 |     match reg {
    |           ^^^ pattern `Avr(_)` not covered
   ::: /checkout/compiler/rustc_target/src/asm/mod.rs:407:5
    |
    |
407 |     Avr(AvrInlineAsmRegClass),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_codegen_gcc` due to 3 previous errors
Build completed unsuccessfully in 0:03:37
