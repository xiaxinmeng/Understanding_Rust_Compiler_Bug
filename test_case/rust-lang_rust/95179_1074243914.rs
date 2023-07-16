plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> compiler/rustc_trait_selection/src/traits/fulfill.rs:583:38
    |
583 | ...                   if infcx.try_unify_abstract_consts(a.shrink(), b.shrink()) {
    |                                |
    |                                expected 3 arguments
    |
note: associated function defined here
---
    |
865 |         try_unify_abstract_consts: const_evaluatable::try_unify_abstract_consts,
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ incorrect number of function parameters
    |
    = note: expected fn pointer `for<'tcx> fn(rustc_middle::ty::TyCtxt<'tcx>, ParamEnvAnd<'tcx, (rustc_middle::ty::Unevaluated<'tcx, ()>, rustc_middle::ty::Unevaluated<'tcx, ()>)>) -> _`
                  found fn item `for<'tcx> fn(rustc_middle::ty::TyCtxt<'tcx>, (rustc_middle::ty::Unevaluated<'tcx, ()>, rustc_middle::ty::Unevaluated<'tcx, ()>), rustc_middle::ty::ParamEnv<'tcx>) -> _ {const_evaluatable::try_unify_abstract_consts}`
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> compiler/rustc_trait_selection/src/traits/select/mod.rs:646:43
    |
    |
646 | ...                   if self.infcx.try_unify_abstract_consts(a.shrink(), b.shrink()) {
    |                                     |
    |                                     expected 3 arguments
    |
note: associated function defined here
