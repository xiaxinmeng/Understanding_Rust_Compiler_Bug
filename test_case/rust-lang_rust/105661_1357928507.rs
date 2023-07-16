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
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0063]: missing field `_use_mk_alias_ty_instead` in initializer of `ty::sty::AliasTy<'_>`
     |
1215 |         AliasTy {
1215 |         AliasTy {
     |         ^^^^^^^ missing `_use_mk_alias_ty_instead`

error[E0592]: duplicate definitions with name `with_self_ty`
     |
     |
1037 |     pub fn with_self_ty(self, tcx: TyCtxt<'tcx>, self_ty: Ty<'tcx>) -> ProjectionPredicate<'tcx> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `with_self_ty`
...
1090 |     pub fn with_self_ty(self, tcx: TyCtxt<'tcx>, self_ty: Ty<'tcx>) -> Self {
     |     ----------------------------------------------------------------------- other definition for `with_self_ty`
Some errors have detailed explanations: E0063, E0592.
For more information about an error, try `rustc --explain E0063`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
