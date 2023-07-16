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
error[E0061]: this function takes 3 arguments but 4 arguments were supplied
    --> compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1310:18
     |
1310 |             self.overwrite_local_ty_if_err(decl.hir_id, decl.pat, decl_ty, init_ty);
     |
note: associated function defined here
    --> compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1626:8
     |
     |
1626 |     fn overwrite_local_ty_if_err(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^
1627 |         &self,
1628 |         hir_id: hir::HirId,
     |         ------------------
1629 |         pat: &'tcx hir::Pat<'tcx>,
1630 |         ty: Ty<'tcx>,
     |         ------------
help: remove the extra argument
     |
     |
1310 |             self.overwrite_local_ty_if_err(decl.hir_id, decl.pat, decl_ty);

error[E0061]: this function takes 3 arguments but 4 arguments were supplied
    --> compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1325:14
     |
     |
1325 |         self.overwrite_local_ty_if_err(decl.hir_id, decl.pat, decl_ty, pat_ty);
     |
note: associated function defined here
    --> compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs:1626:8
     |
     |
1626 |     fn overwrite_local_ty_if_err(
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^
1627 |         &self,
1628 |         hir_id: hir::HirId,
     |         ------------------
1629 |         pat: &'tcx hir::Pat<'tcx>,
1630 |         ty: Ty<'tcx>,
     |         ------------
help: remove the extra argument
     |
     |
1325 |         self.overwrite_local_ty_if_err(decl.hir_id, decl.pat, decl_ty);

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_hir_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
