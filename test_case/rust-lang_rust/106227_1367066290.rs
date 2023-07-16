plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0004]: non-exhaustive patterns: `&rustc_middle::mir::StatementKind::ConstEvalCounter` not covered
   --> src/base.rs:515:11
    |
515 |     match &stmt.kind {
    |           ^^^^^^^^^^ pattern `&rustc_middle::mir::StatementKind::ConstEvalCounter` not covered
note: `rustc_middle::mir::StatementKind<'_>` defined here
   --> /checkout/compiler/rustc_middle/src/mir/syntax.rs:361:5
    |
242 | pub enum StatementKind<'tcx> {
242 | pub enum StatementKind<'tcx> {
    | ----------------------------
...
361 |     ConstEvalCounter,
    |     ^^^^^^^^^^^^^^^^ not covered
    = note: the matched value is of type `&rustc_middle::mir::StatementKind<'_>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
822 ~         },
822 ~         },
823 ~         &rustc_middle::mir::StatementKind::ConstEvalCounter => todo!(),

error[E0004]: non-exhaustive patterns: `&rustc_middle::mir::StatementKind::ConstEvalCounter` not covered
   --> src/constant.rs:481:27
    |
    |
481 |                     match &stmt.kind {
    |                           ^^^^^^^^^^ pattern `&rustc_middle::mir::StatementKind::ConstEvalCounter` not covered
note: `rustc_middle::mir::StatementKind<'_>` defined here
   --> /checkout/compiler/rustc_middle/src/mir/syntax.rs:361:5
    |
242 | pub enum StatementKind<'tcx> {
242 | pub enum StatementKind<'tcx> {
    | ----------------------------
...
361 |     ConstEvalCounter,
    |     ^^^^^^^^^^^^^^^^ not covered
    = note: the matched value is of type `&rustc_middle::mir::StatementKind<'_>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
533 ~                         | StatementKind::Nop => {}
533 ~                         | StatementKind::Nop => {}
534 +                         &rustc_middle::mir::StatementKind::ConstEvalCounter => todo!()

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_codegen_cranelift` due to 2 previous errors
Build completed unsuccessfully in 0:02:12
