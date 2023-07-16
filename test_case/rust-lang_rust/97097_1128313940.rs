plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#bdecdecf)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#bdecdecf)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0004]: non-exhaustive patterns: `X86(tmm_reg)` not covered
    |
    |
552 |         InlineAsmRegOrRegClass::RegClass(reg) => match reg {
    |                                                        ^^^ pattern `X86(tmm_reg)` not covered
note: `rustc_target::asm::InlineAsmRegClass` defined here
   --> /checkout/compiler/rustc_target/src/asm/mod.rs:422:5
    |
421 | / pub enum InlineAsmRegClass {
421 | / pub enum InlineAsmRegClass {
422 | |     X86(X86InlineAsmRegClass),
    | |     ^^^ not covered
423 | |     Arm(ArmInlineAsmRegClass),
424 | |     AArch64(AArch64InlineAsmRegClass),
437 | |     Err,
438 | | }
    | |_-
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
602 ~             InlineAsmRegClass::Err => unreachable!(),
602 ~             InlineAsmRegClass::Err => unreachable!(),
603 ~             X86(tmm_reg) => todo!(),


error[E0004]: non-exhaustive patterns: `X86(tmm_reg)` not covered
    |
612 |     match reg {
612 |     match reg {
    |           ^^^ pattern `X86(tmm_reg)` not covered
note: `rustc_target::asm::InlineAsmRegClass` defined here
   --> /checkout/compiler/rustc_target/src/asm/mod.rs:422:5
    |
421 | / pub enum InlineAsmRegClass {
421 | / pub enum InlineAsmRegClass {
422 | |     X86(X86InlineAsmRegClass),
    | |     ^^^ not covered
423 | |     Arm(ArmInlineAsmRegClass),
424 | |     AArch64(AArch64InlineAsmRegClass),
437 | |     Err,
438 | | }
    | |_-
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
665 ~         InlineAsmRegClass::Err => unreachable!(),
665 ~         InlineAsmRegClass::Err => unreachable!(),
666 ~         X86(tmm_reg) => todo!(),


error[E0004]: non-exhaustive patterns: `X86(tmm_reg)` not covered
    |
738 |     match reg {
738 |     match reg {
    |           ^^^ pattern `X86(tmm_reg)` not covered
note: `rustc_target::asm::InlineAsmRegClass` defined here
   --> /checkout/compiler/rustc_target/src/asm/mod.rs:422:5
    |
421 | / pub enum InlineAsmRegClass {
421 | / pub enum InlineAsmRegClass {
422 | |     X86(X86InlineAsmRegClass),
    | |     ^^^ not covered
423 | |     Arm(ArmInlineAsmRegClass),
424 | |     AArch64(AArch64InlineAsmRegClass),
437 | |     Err,
438 | | }
    | |_-
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
799 ~         InlineAsmRegClass::Err => unreachable!(),
799 ~         InlineAsmRegClass::Err => unreachable!(),
800 ~         X86(tmm_reg) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_codegen_gcc` due to 3 previous errors
Build completed unsuccessfully in 0:03:36
