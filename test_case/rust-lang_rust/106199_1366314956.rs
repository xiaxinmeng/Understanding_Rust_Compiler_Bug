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
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused variable: `decl_ty`
    --> compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1630:9
1630 |         decl_ty: Ty<'tcx>,
1630 |         decl_ty: Ty<'tcx>,
     |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_decl_ty`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_hir_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_typeck` due to previous error
Build completed unsuccessfully in 0:01:29
