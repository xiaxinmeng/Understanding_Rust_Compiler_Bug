
>    Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
> error[E0061]: this function takes 2 arguments but 1 argument was supplied
>     --> compiler/rustc_middle/src/ty/context.rs:1313:14
>      |
> 1313 |         self.mk_const(ty::ConstS { kind: ty::ConstKind::Error(reported), ty })
>      |                      ||
>      |                      ||
>      |                      |expected enum `kind::ConstKind`, found struct `ConstS`
>      |
> note: associated function defined here
>     --> compiler/rustc_middle/src/ty/context.rs:2589:12
>      |
>      |
> 2589 |     pub fn mk_const(self, kind: ty::ConstKind<'tcx>, ty: Ty<'tcx>) -> Const<'tcx> {
> help: provide the argument
>      |
>      |
> 1313 |         self.mk_const(/* kind::ConstKind<'_> */, /* ty::Ty<'_> */)
> 
> [RUSTC-TIMING] rustc_ast_passes test:false 4.110
> For more information about this error, try `rustc --explain E0061`.
> [RUSTC-TIMING] rustc_middle test:false 7.533
> 