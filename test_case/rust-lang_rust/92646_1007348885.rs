plain
    |
465 | #[pass_by_value]
    |   ^^^^^^^^^^^^^

error: cannot determine resolution for the derive macro `TypeFoldable`
    --> compiler/rustc_middle/src/ty/sty.rs:1539:22
     |
1539 | #[derive(HashStable, TypeFoldable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
    --> compiler/rustc_middle/src/ty/sty.rs:1244:22
     |
1244 | #[derive(HashStable, TypeFoldable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
    --> compiler/rustc_middle/src/ty/sty.rs:1228:30
     |
1228 | #[derive(Copy, Clone, Debug, TypeFoldable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
    --> compiler/rustc_middle/src/ty/sty.rs:1179:22
     |
1179 | #[derive(HashStable, TypeFoldable)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
   --> compiler/rustc_middle/src/ty/sty.rs:959:22
    |
959 | #[derive(HashStable, TypeFoldable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
   --> compiler/rustc_middle/src/ty/sty.rs:895:22
    |
895 | #[derive(HashStable, TypeFoldable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
   --> compiler/rustc_middle/src/ty/sty.rs:769:22
    |
769 | #[derive(HashStable, TypeFoldable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
   --> compiler/rustc_middle/src/ty/sty.rs:722:30
    |
722 | #[derive(Copy, Clone, Debug, TypeFoldable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
   --> compiler/rustc_middle/src/ty/sty.rs:444:30
    |
444 | #[derive(Copy, Clone, Debug, TypeFoldable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
   --> compiler/rustc_middle/src/ty/sty.rs:317:30
    |
317 | #[derive(Copy, Clone, Debug, TypeFoldable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
  --> compiler/rustc_middle/src/ty/sty.rs:29:22
   |
29 | #[derive(HashStable, TypeFoldable, Lift)]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
  --> compiler/rustc_middle/src/ty/instance.rs:27:48
   |
27 | #[derive(TyEncodable, TyDecodable, HashStable, TypeFoldable)]
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `Lift`
   --> compiler/rustc_middle/src/ty/subst.rs:705:36
    |
705 | #[derive(HashStable, TypeFoldable, Lift)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `Lift`
   --> compiler/rustc_middle/src/ty/subst.rs:678:36
    |
678 | #[derive(HashStable, TypeFoldable, Lift)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
   --> compiler/rustc_middle/src/ty/relate.rs:343:30
    |
343 | #[derive(Copy, Debug, Clone, TypeFoldable)]
    |
    |
    = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
    --> compiler/rustc_middle/src/ty/print/pretty.rs:2385:23
     |
2385 | #[derive(Copy, Clone, TypeFoldable, Lift)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the derive macro `TypeFoldable`
    --> compiler/rustc_middle/src/ty/print/pretty.rs:2373:23
     |
2373 | #[derive(Copy, Clone, TypeFoldable, Lift)]
     |
     |
     = note: import resolution is stuck, try simplifying macro imports
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `sty::FnSig<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:168:12
    |
168 | impl<'tcx> Relate<'tcx> for ty::FnSig<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `sty::FnSig<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`

error[E0277]: the trait bound `sty::ProjectionTy<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:262:12
    |
262 | impl<'tcx> Relate<'tcx> for ty::ProjectionTy<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `sty::ProjectionTy<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`

error[E0277]: the trait bound `ExistentialProjection<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:281:12
    |
281 | impl<'tcx> Relate<'tcx> for ty::ExistentialProjection<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `ExistentialProjection<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`

error[E0277]: the trait bound `sty::TraitRef<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:311:12
    |
311 | impl<'tcx> Relate<'tcx> for ty::TraitRef<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `sty::TraitRef<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`

error[E0277]: the trait bound `ExistentialTraitRef<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:327:12
    |
327 | impl<'tcx> Relate<'tcx> for ty::ExistentialTraitRef<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `ExistentialTraitRef<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`

error[E0277]: the trait bound `relate::GeneratorWitness<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:346:12
    |
346 | impl<'tcx> Relate<'tcx> for GeneratorWitness<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `relate::GeneratorWitness<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`

error[E0277]: the trait bound `ClosureSubsts<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:717:12
    |
717 | impl<'tcx> Relate<'tcx> for ty::ClosureSubsts<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `ClosureSubsts<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`

error[E0277]: the trait bound `GeneratorSubsts<'tcx>: ty::fold::TypeFoldable<'tcx>` is not satisfied
   --> compiler/rustc_middle/src/ty/relate.rs:728:12
    |
728 | impl<'tcx> Relate<'tcx> for ty::GeneratorSubsts<'tcx> {
    |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `GeneratorSubsts<'tcx>`
note: required by a bound in `Relate`
   --> compiler/rustc_middle/src/ty/relate.rs:105:25
    |
    |
105 | pub trait Relate<'tcx>: TypeFoldable<'tcx> + Copy {
    |                         ^^^^^^^^^^^^^^^^^^ required by this bound in `Relate`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 26 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
