plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_mir_transform/src/shim.rs:345:26
     |
345  |         let substs = tcx.mk_substs_trait(self_ty, &[]);
     |                          |
     |                          expected 3 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:2691:12
     |
2691 |     pub fn mk_substs_trait(

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_mir_transform/src/shim.rs:426:26
     |
     |
426  |         let substs = tcx.mk_substs_trait(ty, &[]);
     |                          |
     |                          expected 3 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:2691:12
     |
2691 |     pub fn mk_substs_trait(

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> compiler/rustc_mir_transform/src/shim.rs:531:30
     |
     |
531  |         let sig_substs = tcx.mk_substs_trait(ty, &[ty::subst::GenericArg::from(arg_tup)]);
     |                              |
     |                              expected 3 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:2691:12
     |
2691 |     pub fn mk_substs_trait(

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_mir_transform` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_mir_transform` due to 3 previous errors
error[E0004]: non-exhaustive patterns: `(Constness(_), _)` not covered
    |
    |
608 |             match (kind.unpack(), hir_arg) {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^ pattern `(Constness(_), _)` not covered
    |
    = note: the matched value is of type `(GenericArgKind, &rustc_hir::GenericArg)`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
636 ~                 }
637 + 
637 + 
638 +                 (Constness(_), _) => todo!()


error[E0004]: non-exhaustive patterns: `Constness(_)` not covered
    |
    |
222 |         let arg_is_param = match arg.unpack() {
    |                                  ^^^^^^^^^^^^ pattern `Constness(_)` not covered
    |
note: `GenericArgKind` defined here
   --> /checkout/compiler/rustc_middle/src/ty/subst.rs:50:5
    |
46  | / pub enum GenericArgKind<'tcx> {
47  | |     Lifetime(ty::Region<'tcx>),
48  | |     Type(Ty<'tcx>),
49  | |     Const(ty::Const<'tcx>),
50  | |     Constness(ty::ConstnessArg),
51  | | }
    | |_-
    | |_-
    = note: the matched value is of type `GenericArgKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
239 ~             GenericArgKind::Const(ct) => matches!(ct.val(), ty::ConstKind::Param(_)),
240 ~             Constness(_) => todo!(),


error[E0004]: non-exhaustive patterns: `Constness(_)` not covered
   |
   |
95 |         match k1.unpack() {
   |               ^^^^^^^^^^^ pattern `Constness(_)` not covered
   |
note: `GenericArgKind` defined here
  --> /checkout/compiler/rustc_middle/src/ty/subst.rs:50:5
   |
46 | / pub enum GenericArgKind<'tcx> {
47 | |     Lifetime(ty::Region<'tcx>),
48 | |     Type(Ty<'tcx>),
49 | |     Const(ty::Const<'tcx>),
50 | |     Constness(ty::ConstnessArg),
51 | | }
   | |_-
   | |_-
   = note: the matched value is of type `GenericArgKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
132~             }
133+ 
133+ 
134+             Constness(_) => todo!()


error[E0004]: non-exhaustive patterns: `Constness(_)` not covered
     |
     |
2528 |                     match k1.unpack() {
     |                           ^^^^^^^^^^^ pattern `Constness(_)` not covered
     |
note: `GenericArgKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/subst.rs:50:5
     |
46   | / pub enum GenericArgKind<'tcx> {
47   | |     Lifetime(ty::Region<'tcx>),
48   | |     Type(Ty<'tcx>),
49   | |     Const(ty::Const<'tcx>),
50   | |     Constness(ty::ConstnessArg),
51   | | }
     | |_-
     | |_-
     = note: the matched value is of type `GenericArgKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
     |
2540 ~                         GenericArgKind::Type(_) | GenericArgKind::Const(_) => None,
2541 ~                         Constness(_) => todo!(),

error: unreachable pattern
   --> compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs:415:9
    |
    |
415 |         ty::PredicateKind::Trait(_)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`

error[E0004]: non-exhaustive patterns: `Constness` not covered
     |
     |
1274 |     InternalSubsts::fill_single(&mut substs, defs, &mut |param, _| match param.kind {
     |                                                                          ^^^^^^^^^^ pattern `Constness` not covered
note: `GenericParamDefKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
     |
13   | / pub enum GenericParamDefKind {
13   | / pub enum GenericParamDefKind {
14   | |     Lifetime,
15   | |     Type { has_default: bool, object_lifetime_default: ObjectLifetimeDefault, synthetic: bool },
16   | |     Const { has_default: bool },
17   | |     Constness,
18   | | }
     | |_-
     = note: the matched value is of type `GenericParamDefKind`
     = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1306 ~         }
1306 ~         }
1307 +         Constness => todo!()

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_borrowck` due to 4 previous errors
error: could not compile `rustc_borrowck` due to 4 previous errors
