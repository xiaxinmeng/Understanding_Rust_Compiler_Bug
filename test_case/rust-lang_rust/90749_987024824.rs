plain

error[E0433]: failed to resolve: use of undeclared type `BTreeMap`
   --> compiler/rustc_middle/src/ty/print/pretty.rs:767:29
    |
767 |         let mut fn_traits = BTreeMap::new();
    |
help: consider importing this struct
    |
1   | use std::collections::BTreeMap;
1   | use std::collections::BTreeMap;
    |

error[E0412]: cannot find type `BTreeMap` in this scope
   --> compiler/rustc_middle/src/ty/print/pretty.rs:936:22
    |
936 |         traits: &mut BTreeMap<ty::PolyTraitRef<'tcx>, BTreeMap<DefId, ty::Binder<'tcx, Ty<'tcx>>>>,
    |
help: consider importing this struct
    |
1   | use std::collections::BTreeMap;
1   | use std::collections::BTreeMap;
    |

error[E0412]: cannot find type `BTreeMap` in this scope
   --> compiler/rustc_middle/src/ty/print/pretty.rs:936:55
    |
936 |         traits: &mut BTreeMap<ty::PolyTraitRef<'tcx>, BTreeMap<DefId, ty::Binder<'tcx, Ty<'tcx>>>>,
    |
help: consider importing this struct
    |
1   | use std::collections::BTreeMap;
1   | use std::collections::BTreeMap;
    |

error[E0412]: cannot find type `BTreeMap` in this scope
   --> compiler/rustc_middle/src/ty/print/pretty.rs:937:25
    |
937 |         fn_traits: &mut BTreeMap<ty::PolyTraitRef<'tcx>, OpaqueFnEntry<'tcx>>,
    |
help: consider importing this struct
    |
1   | use std::collections::BTreeMap;
1   | use std::collections::BTreeMap;
    |

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
error[E0614]: type `rustc_span::def_id::DefId` cannot be dereferenced
   --> compiler/rustc_middle/src/ty/print/pretty.rs:903:37
    |
    |
903 | ...                   if Some(*item_def_id) == self.tcx().lang_items().generator_return() =>


error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/sty.rs:899:5
     |
896  |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TyEncodable, TyDecodable)]
     |                                        ---------- in this derive macro expansion
899  |       pub def_id: DefId,
899  |       pub def_id: DefId,
     |       ^^^^^^^^^^^^^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `subst::GenericArg<'_>` with `subst::GenericArg<'_>`
    --> compiler/rustc_middle/src/ty/sty.rs:900:5
     |
896  |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TyEncodable, TyDecodable)]
     |                                        ---------- in this derive macro expansion
...
900  |       pub substs: SubstsRef<'tcx>,
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `subst::GenericArg<'_> < subst::GenericArg<'_>` and `subst::GenericArg<'_> > subst::GenericArg<'_>`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `subst::GenericArg<'_>`
note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `list::List<subst::GenericArg<'_>>`
     |
119  | impl<T> PartialOrd for List<T>
     |         ^^^^^^^^^^     ^^^^^^^
     |         ^^^^^^^^^^     ^^^^^^^
     = note: 1 redundant requirement hidden
     = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&list::List<subst::GenericArg<'_>>>` for `&list::List<subst::GenericArg<'_>>`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/sty.rs:899:5
    |
896 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TyEncodable, TyDecodable)]
    |                                                    --- in this derive macro expansion
899 |       pub def_id: DefId,
899 |       pub def_id: DefId,
    |       ^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
   ::: /checkout/library/core/src/cmp.rs:831:1
    |
    |
831 | / pub macro Ord($item:item) {
833 | | }
833 | | }
    | |_- in this expansion of `#[derive(Ord)]`

error[E0277]: the trait bound `subst::GenericArg<'_>: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/sty.rs:900:5
    |
896 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TyEncodable, TyDecodable)]
    |                                                    --- in this derive macro expansion
...
900 |       pub substs: SubstsRef<'tcx>,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `subst::GenericArg<'_>`
   ::: /checkout/library/core/src/cmp.rs:831:1
    |
    |
831 | / pub macro Ord($item:item) {
833 | | }
833 | | }
    | |_- in this expansion of `#[derive(Ord)]`
    |
note: required because of the requirements on the impl of `Ord` for `list::List<subst::GenericArg<'_>>`
    |
    |
110 | impl<T> Ord for List<T>
    |         ^^^     ^^^^^^^
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Ord` for `&list::List<subst::GenericArg<'_>>`
Some errors have detailed explanations: E0277, E0412, E0433, E0614.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 11 previous errors
warning: build failed, waiting for other jobs to finish...
