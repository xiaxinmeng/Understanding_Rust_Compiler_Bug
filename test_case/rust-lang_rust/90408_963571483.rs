plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0308]: mismatched types
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:408:60
    |
408 |             let upvar_def_span = self.infcx.tcx.hir().span(upvar);
    |                                                            ^^^^^ expected struct `HirId`, found struct `rustc_middle::mir::Field`

error[E0277]: the trait bound `HirId: std::borrow::Borrow<rustc_middle::mir::Field>` is not satisfied
    |
    |
409 |             let upvar_span = self.infcx.tcx.upvars_mentioned(def_id).unwrap()[&upvar].span;
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::borrow::Borrow<rustc_middle::mir::Field>` is not implemented for `HirId`
    |
    = note: required because of the requirements on the impl of `indexmap::equivalent::Equivalent<HirId>` for `rustc_middle::mir::Field`
    = note: required because of the requirements on the impl of `std::ops::Index<&rustc_middle::mir::Field>` for `indexmap::map::IndexMap<HirId, rustc_hir::Upvar, BuildHasherDefault<FxHasher>>`
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_borrowck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0277]: can't compare `HirId` with `HirId`
     |
     |
89   |   #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
     |                                         ---------- in this derive macro expansion
...
92   |       CapturingPrecise { source_expr: Option<hir::HirId>, var_name: String },
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `HirId < HirId` and `HirId > HirId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `HirId`
     = note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `Option<HirId>`
note: required by `std::cmp::PartialOrd::partial_cmp`
     |
     |
1018 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;


error[E0277]: the trait bound `HirId: Ord` is not satisfied
    |
    |
89  |   #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    |                                                     --- in this derive macro expansion
...
92  |       CapturingPrecise { source_expr: Option<hir::HirId>, var_name: String },
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `HirId`
   ::: /checkout/library/core/src/cmp.rs:831:1
    |
    |
831 | / pub macro Ord($item:item) {
833 | | }
833 | | }
    | |_- in this expansion of `#[derive(Ord)]`
    |
    = note: required because of the requirements on the impl of `Ord` for `Option<HirId>`
note: required by `std::cmp::Ord::cmp`
    |
    |
752 |     fn cmp(&self, other: &Self) -> Ordering;

error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/upvar.rs:1191:21
     |
     |
1191 |                 let (o, s) = c.clone();
     |                     ^^^^^^   --------- this expression has type `UpvarMigrationInfo`
     |                     |
     |                     expected enum `UpvarMigrationInfo`, found tuple
     |
     = note: expected enum `UpvarMigrationInfo`
               found tuple `(_, _)`
error: build failed
Build completed unsuccessfully in 0:02:46
