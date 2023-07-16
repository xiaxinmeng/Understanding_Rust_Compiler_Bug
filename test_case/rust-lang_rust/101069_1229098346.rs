plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#bdb86fb5)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#bdb86fb5)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0004]: non-exhaustive patterns: `LoongArch(_)` not covered
    |
    |
567 |         InlineAsmRegOrRegClass::RegClass(reg) => match reg {
    |                                                        ^^^ pattern `LoongArch(_)` not covered
note: `rustc_target::asm::InlineAsmRegClass` defined here
   --> /checkout/compiler/rustc_target/src/asm/mod.rs:420:5
    |
412 | pub enum InlineAsmRegClass {
412 | pub enum InlineAsmRegClass {
    | --------------------------
...
420 |     LoongArch(LoongArchInlineAsmRegClass),
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
617 ~             InlineAsmRegClass::Err => unreachable!(),
617 ~             InlineAsmRegClass::Err => unreachable!(),
618 ~             LoongArch(_) => todo!(),


error[E0004]: non-exhaustive patterns: `LoongArch(_)` not covered
    |
627 |     match reg {
627 |     match reg {
    |           ^^^ pattern `LoongArch(_)` not covered
note: `rustc_target::asm::InlineAsmRegClass` defined here
   --> /checkout/compiler/rustc_target/src/asm/mod.rs:420:5
    |
412 | pub enum InlineAsmRegClass {
412 | pub enum InlineAsmRegClass {
    | --------------------------
...
420 |     LoongArch(LoongArchInlineAsmRegClass),
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
681 ~         InlineAsmRegClass::Err => unreachable!(),
681 ~         InlineAsmRegClass::Err => unreachable!(),
682 ~         LoongArch(_) => todo!(),


error[E0004]: non-exhaustive patterns: `LoongArch(_)` not covered
    |
754 |     match reg {
754 |     match reg {
    |           ^^^ pattern `LoongArch(_)` not covered
note: `rustc_target::asm::InlineAsmRegClass` defined here
   --> /checkout/compiler/rustc_target/src/asm/mod.rs:420:5
    |
412 | pub enum InlineAsmRegClass {
412 | pub enum InlineAsmRegClass {
    | --------------------------
...
420 |     LoongArch(LoongArchInlineAsmRegClass),
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
    = note: the matched value is of type `rustc_target::asm::InlineAsmRegClass`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
815 ~         InlineAsmRegClass::Err => unreachable!(),
815 ~         InlineAsmRegClass::Err => unreachable!(),
816 ~         LoongArch(_) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_codegen_gcc` due to 3 previous errors
Build completed unsuccessfully in 0:03:59
