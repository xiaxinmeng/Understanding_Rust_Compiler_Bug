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
error[E0271]: type mismatch resolving `<&[_; 0] as IntoIterator>::Item == subst::GenericArg<'_>`
     |
     |
676  | ...                   tcx.mk_projection(metadata_def_id, tcx.mk_substs_trait(pointee, &[])),
     |                                                              ---------------          ^^^ expected struct `subst::GenericArg`, found reference
     |                                                              required by a bound introduced by this call
     |
     |
     = note: expected struct `subst::GenericArg<'_>`
             found reference `&_`
note: required by a bound in `context::TyCtxt::<'tcx>::mk_substs_trait`
     |
     |
2195 |     pub fn mk_substs_trait(
...
...
2198 |         rest: impl IntoIterator<Item = GenericArg<'tcx>>,
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `context::TyCtxt::<'tcx>::mk_substs_trait`
For more information about this error, try `rustc --explain E0271`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
