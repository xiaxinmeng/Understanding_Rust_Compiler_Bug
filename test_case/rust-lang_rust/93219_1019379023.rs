plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0004]: non-exhaustive patterns: `Msp430(_)` not covered
    |
    |
544 |         InlineAsmRegOrRegClass::RegClass(reg) => match reg {
    |                                                        ^^^ pattern `Msp430(_)` not covered
   ::: /checkout/compiler/rustc_target/src/asm/mod.rs:420:5
    |
    |
420 |     Msp430(Msp430InlineAsmRegClass),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`


error[E0004]: non-exhaustive patterns: `Msp430(_)` not covered
    |
602 |     match reg {
602 |     match reg {
    |           ^^^ pattern `Msp430(_)` not covered
   ::: /checkout/compiler/rustc_target/src/asm/mod.rs:420:5
    |
    |
420 |     Msp430(Msp430InlineAsmRegClass),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`


error[E0004]: non-exhaustive patterns: `Msp430(_)` not covered
    |
710 |     match reg {
710 |     match reg {
    |           ^^^ pattern `Msp430(_)` not covered
   ::: /checkout/compiler/rustc_target/src/asm/mod.rs:420:5
    |
    |
420 |     Msp430(Msp430InlineAsmRegClass),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`

