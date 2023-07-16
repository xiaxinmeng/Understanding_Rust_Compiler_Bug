
[00:06:22]    Compiling rustc v0.0.0 (file:///checkout/src/librustc)
[00:06:35] error: no method named `parameter_environment` found for type `ty::context::TyCtxt<'a, 'tcx, 'tcx>` in the current scope
[00:06:35]    --> /checkout/src/librustc/middle/effect.rs:231:54
[00:06:35]     |
[00:06:35] 231 |                             let param_env = self.tcx.parameter_environment(owner_def_id);
[00:06:35]     |                                                      ^^^^^^^^^^^^^^^^^^^^^
[00:06:35] 
[00:06:35] error[E0308]: mismatched types
[00:06:35]    --> /checkout/src/librustc/middle/effect.rs:232:68
[00:06:35]     |
[00:06:35] 232 |                             if field_ty.moves_by_default(self.tcx, &param_env, field.span) {
[00:06:35]     |                                                                    ^^^^^^^^^^ expected struct `ty::ParamEnv`, found reference
[00:06:35]     |
[00:06:35]     = note: expected type `ty::ParamEnv<'_>`
[00:06:35]                found type `&_`
[00:06:35] 
[00:06:42] error: aborting due to 2 previous errors
[00:06:42] 
[00:06:42] error: Could not compile `rustc`.
