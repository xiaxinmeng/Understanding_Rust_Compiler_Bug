plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/traits/chalk.rs:77:5
77  |     type DefId = DefId;
77  |     type DefId = DefId;
    |     ^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    |
note: required by a bound in `chalk_ir::interner::Interner::DefId`
    |
    |
191 |     type DefId: Debug + Copy + Eq + Ord + Hash;
    |                                     ^^^ required by this bound in `chalk_ir::interner::Interner::DefId`

error[E0277]: the trait bound `adt::AdtDef: Ord` is not satisfied
   --> compiler/rustc_middle/src/traits/chalk.rs:78:5
    |
78  |     type InternedAdtId = &'tcx AdtDef;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `adt::AdtDef`
    |
    = note: required because of the requirements on the impl of `Ord` for `&'tcx adt::AdtDef`
note: required by a bound in `chalk_ir::interner::Interner::InternedAdtId`
    |
    |
194 |     type InternedAdtId: Debug + Copy + Eq + Ord + Hash;
    |                                             ^^^ required by this bound in `chalk_ir::interner::Interner::InternedAdtId`
error[E0308]: mismatched types
   --> compiler/rustc_middle/src/hir/mod.rs:109:49
    |
    |
109 |     providers.all_local_trait_impls = |tcx, ()| &tcx.resolutions(()).trait_impls;
    |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `HashMap`, found struct `BTreeMap`
    |
    = note: expected reference `&HashMap<rustc_span::def_id::DefId, Vec<rustc_span::def_id::LocalDefId>, BuildHasherDefault<FxHasher>>`
               found reference `&BTreeMap<rustc_span::def_id::DefId, Vec<rustc_span::def_id::LocalDefId>>`

error[E0277]: the trait bound `ty::FieldDef: std::cmp::Eq` is not satisfied
    --> compiler/rustc_middle/src/ty/mod.rs:1437:5
     |
1423 |   #[derive(Debug, HashStable, Eq, PartialEq)]
     |                               -- in this derive macro expansion
...
1437 |       pub fields: Vec<FieldDef>,
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::cmp::Eq` is not implemented for `ty::FieldDef`
    ::: /checkout/library/core/src/cmp.rs:293:1
     |
     |
293  | / pub macro Eq($item:item) {
295  | | }
295  | | }
     | |_- in this expansion of `#[derive(Eq)]`
     |
     = note: required because of the requirements on the impl of `std::cmp::Eq` for `Vec<ty::FieldDef>`
note: required by a bound in `AssertParamIsEq`
     |
     |
304  | pub struct AssertParamIsEq<T: Eq + ?Sized> {
     |                               ^^ required by this bound in `AssertParamIsEq`

error[E0369]: binary operation `==` cannot be applied to type `Vec<ty::FieldDef>`
    --> compiler/rustc_middle/src/ty/mod.rs:1437:5
     |
1423 |   #[derive(Debug, HashStable, Eq, PartialEq)]
     |                                   --------- in this derive macro expansion
...
1437 |       pub fields: Vec<FieldDef>,
     |
    ::: /checkout/library/core/src/cmp.rs:227:1
     |
     |
227  | / pub macro PartialEq($item:item) {
229  | | }
229  | | }
     | |_- in this expansion of `#[derive(PartialEq)]`

error[E0369]: binary operation `!=` cannot be applied to type `Vec<ty::FieldDef>`
    --> compiler/rustc_middle/src/ty/mod.rs:1437:5
     |
1423 |   #[derive(Debug, HashStable, Eq, PartialEq)]
     |                                   --------- in this derive macro expansion
...
1437 |       pub fields: Vec<FieldDef>,
     |
    ::: /checkout/library/core/src/cmp.rs:227:1
     |
     |
227  | / pub macro PartialEq($item:item) {
229  | | }
229  | | }
     | |_- in this expansion of `#[derive(PartialEq)]`
Some errors have detailed explanations: E0277, E0308, E0369.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
