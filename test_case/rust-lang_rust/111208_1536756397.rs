plain
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error[E0432]: unresolved import `self::closure::AliasRelationDirection`
   --> compiler/rustc_middle/src/ty/mod.rs:109:63
    |
109 |     is_ancestor_or_same_capture, place_to_string_for_capture, AliasRelationDirection, BorrowKind,
    |                                                               ^^^^^^^^^^^^^^^^^^^^^^ no `AliasRelationDirection` in `ty::closure`
    = help: consider importing this enum instead:
            crate::ty::predicate::AliasRelationDirection

    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `AliasRelationDirection: rustc_type_ir::fold::TypeFoldable<_>` is not satisfied
  --> compiler/rustc_middle/src/ty/predicate.rs:30:22
   |
30 |   #[derive(HashStable, TypeFoldable, TypeVisitable, Lift)]
   |                        |
   |                        |
   |                        the trait `rustc_type_ir::fold::TypeFoldable<_>` is not implemented for `AliasRelationDirection`
   |
  ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/synstructure-0.13.0/src/macros.rs:94:9
   |
94 | /         pub fn $derives(
94 | /         pub fn $derives(
95 | |             i: $crate::macros::TokenStream
96 | |         ) -> $crate::macros::TokenStream {
   | |________________________________________- in this expansion of `#[derive(TypeFoldable)]`
   |
   = help: the following other types implement trait `rustc_type_ir::fold::TypeFoldable<I>`:
             <&'tcx [InlineAsmTemplatePiece] as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>
             <&'tcx [rustc_span::Span] as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>
             <&'tcx list::List<CanonicalVarInfo<'tcx>> as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>
             <&'tcx list::List<Predicate<'tcx>> as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>
             <&'tcx list::List<subst::GenericArg<'tcx>> as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>
             <&'tcx list::List<syntax::ProjectionElem<mir::Local, ty_::Ty<'tcx>>> as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>
             <&'tcx list::List<ty::consts::Const<'tcx>> as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>
             <&'tcx list::List<ty::sty::Binder<'tcx, ExistentialPredicate<'tcx>>> as rustc_type_ir::fold::TypeFoldable<context::TyCtxt<'tcx>>>


error[E0277]: the trait bound `AliasRelationDirection: rustc_type_ir::visit::TypeVisitable<_>` is not satisfied
  --> compiler/rustc_middle/src/ty/predicate.rs:30:36
   |
30 |   #[derive(HashStable, TypeFoldable, TypeVisitable, Lift)]
   |                                      |
   |                                      |
   |                                      the trait `rustc_type_ir::visit::TypeVisitable<_>` is not implemented for `AliasRelationDirection`
   |
  ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/synstructure-0.13.0/src/macros.rs:94:9
   |
94 | /         pub fn $derives(
94 | /         pub fn $derives(
95 | |             i: $crate::macros::TokenStream
96 | |         ) -> $crate::macros::TokenStream {
   | |________________________________________- in this expansion of `#[derive(TypeVisitable)]`
   |
   = help: the following other types implement trait `rustc_type_ir::visit::TypeVisitable<I>`:
             <&'tcx list::List<T> as rustc_type_ir::visit::TypeVisitable<context::TyCtxt<'tcx>>>
             <&[T] as rustc_type_ir::visit::TypeVisitable<I>>
             <() as rustc_type_ir::visit::TypeVisitable<I>>
             <(A, B, C) as rustc_type_ir::visit::TypeVisitable<I>>
             <(T, U) as rustc_type_ir::visit::TypeVisitable<I>>
             <Adjust<'tcx> as rustc_type_ir::visit::TypeVisitable<context::TyCtxt<'tcx>>>
             <Adjustment<'tcx> as rustc_type_ir::visit::TypeVisitable<context::TyCtxt<'tcx>>>
             <AdtDef<'tcx> as rustc_type_ir::visit::TypeVisitable<context::TyCtxt<'tcx>>>


error[E0277]: the trait bound `AliasRelationDirection: Lift<'_>` is not satisfied
   --> compiler/rustc_middle/src/ty/predicate.rs:30:51
    |
30  |   #[derive(HashStable, TypeFoldable, TypeVisitable, Lift)]
    |                                                     |
    |                                                     |
    |                                                     the trait `Lift<'_>` is not implemented for `AliasRelationDirection`
    |
   ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/synstructure-0.13.0/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(Lift)]`
    |
    = help: the following other types implement trait `Lift<'tcx>`:
              <&'a list::List<BoundVariableKind> as Lift<'tcx>>
              <&'a list::List<CanonicalVarInfo<'a>> as Lift<'tcx>>
              <&'a list::List<Predicate<'a>> as Lift<'tcx>>
              <&'a list::List<subst::GenericArg<'a>> as Lift<'tcx>>
              <&'a list::List<syntax::ProjectionElem<(), ()>> as Lift<'tcx>>
              <&'a list::List<ty::sty::Binder<'a, ExistentialPredicate<'a>>> as Lift<'tcx>>
              <&'a list::List<ty_::Ty<'a>> as Lift<'tcx>>
              <() as Lift<'tcx>>
note: required by a bound in `context::TyCtxt::<'tcx>::lift`
   --> compiler/rustc_middle/src/ty/context.rs:641:20
    |
    |
641 |     pub fn lift<T: Lift<'tcx>>(self, value: T) -> Option<T::Lifted> {
    |                    ^^^^^^^^^^ required by this bound in `TyCtxt::<'tcx>::lift`
Some errors have detailed explanations: E0277, E0432.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` (lib test) due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
