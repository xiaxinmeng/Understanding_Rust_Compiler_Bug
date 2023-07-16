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
    --> compiler/rustc_trait_selection/src/traits/error_reporting/suggestions.rs:214:6
     |
214  |     );
     |      ^
     = note: expected fn pointer `fn(&TypeErrCtxt<'_, 'tcx>, &rustc_infer::traits::Obligation<'tcx, rustc_middle::ty::Predicate<'tcx>>, &mut Diagnostic, Binder<'_, _>)`
                found fn pointer `fn(&TypeErrCtxt<'_, 'tcx>, &rustc_infer::traits::Obligation<'tcx, rustc_middle::ty::Predicate<'tcx>>, &mut Diagnostic, Binder<'_, _>) -> bool`
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:876:28
    |
    |
876 |                         if self.suggest_add_clone_to_arg(&obligation, &mut err, trait_predicate) {
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
Some errors have detailed explanations: E0053, E0308.
For more information about an error, try `rustc --explain E0053`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
