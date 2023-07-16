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
    Checking cranelift-native v0.90.1
    Checking cranelift-frontend v0.90.1
    Checking cranelift-object v0.90.1
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0004]: non-exhaustive patterns: `rustc_middle::mir::Rvalue::Cast(rustc_middle::mir::CastKind::StripPattern, _, _)` and `rustc_middle::mir::Rvalue::Cast(rustc_middle::mir::CastKind::Patternize, _, _)` not covered
     |
     |
523  |             match to_place_and_rval.1 {
     |                   ^^^^^^^^^^^^^^^^^^^ patterns `rustc_middle::mir::Rvalue::Cast(rustc_middle::mir::CastKind::StripPattern, _, _)` and `rustc_middle::mir::Rvalue::Cast(rustc_middle::mir::CastKind::Patternize, _, _)` not covered
     |
note: `rustc_middle::mir::Rvalue<'_>` defined here
     |
1026 | pub enum Rvalue<'tcx> {
     | ---------------------
...
...
1084 |     Cast(CastKind, Operand<'tcx>, Ty<'tcx>),
     = note: the matched value is of type `rustc_middle::mir::Rvalue<'_>`
     = note: the matched value is of type `rustc_middle::mir::Rvalue<'_>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
786  ~                 },
786  ~                 },
787  ~                 rustc_middle::mir::Rvalue::Cast(rustc_middle::mir::CastKind::StripPattern, _, _) | rustc_middle::mir::Rvalue::Cast(rustc_middle::mir::CastKind::Patternize, _, _) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:02:33
