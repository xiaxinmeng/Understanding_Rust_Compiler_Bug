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
error[E0308]: mismatched types
   --> src/base.rs:855:48
    |
855 |                 let index = fx.get_local_place(local).to_cvalue(fx).load_scalar(fx);
    |                                --------------- ^^^^^ expected struct `rustc_middle::mir::Local`, found struct `rustc_middle::mir::Place`
    |                                arguments to this function are incorrect
    |
note: associated function defined here
   --> src/common.rs:350:19
   --> src/common.rs:350:19
    |
350 |     pub(crate) fn get_local_place(&mut self, local: Local) -> CPlace<'tcx> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:02:43
