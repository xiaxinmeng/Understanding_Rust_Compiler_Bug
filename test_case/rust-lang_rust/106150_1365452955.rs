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
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0423]: expected value, found macro `error`
  --> compiler/rustc_hir_typeck/src/demand.rs:40:59
   |
40 |         self.annotate_alternative_method_deref(err, expr, error);
   |                                                           |
   |                                                           not a value
   |                                                           not a value
   |                                                           help: a local variable with a similar name exists: `_error`
For more information about this error, try `rustc --explain E0423`.
error: could not compile `rustc_hir_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_typeck` due to previous error
