plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `BasicBlocks<'tcx>: ty::visit::TypeVisitable<'tcx>` is not satisfied
   |
   |
11 |   #[derive(Clone, TyEncodable, TyDecodable, Debug, HashStable, TypeFoldable)]
   |                                                                |
   |                                                                |
   |                                                                the trait `ty::visit::TypeVisitable<'tcx>` is not implemented for `BasicBlocks<'tcx>`
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
   |
94 | /         pub fn $derives(
94 | /         pub fn $derives(
95 | |             i: $crate::macros::TokenStream
96 | |         ) -> $crate::macros::TokenStream {
   | |________________________________________- in this expansion of `#[derive(TypeFoldable)]`
   |
   = help: the following other types implement trait `ty::visit::TypeVisitable<'tcx>`:
             &'tcx list::List<Binder<'tcx, ExistentialPredicate<'tcx>>>
             &'tcx list::List<CanonicalVarInfo<'tcx>>
             &'tcx list::List<Predicate<'tcx>>
             &'tcx list::List<subst::GenericArg<'tcx>>
             &'tcx list::List<syntax::ProjectionElem<(), ()>>
             &'tcx list::List<syntax::ProjectionElem<mir::Local, ty::Ty<'tcx>>>
             &'tcx list::List<ty::Ty<'tcx>>
           and 212 others
note: required by a bound in `ty::fold::TypeFoldable`
  --> compiler/rustc_middle/src/ty/fold.rs:57:31
   |
   |
57 | pub trait TypeFoldable<'tcx>: TypeVisitable<'tcx> {
   |                               ^^^^^^^^^^^^^^^^^^^ required by this bound in `ty::fold::TypeFoldable`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
