plain
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/hir/map/mod.rs:552:48
    |
552 |         self.tcx.all_local_trait_impls(()).get(&trait_did).map_or(&[], |xs| &xs[..])
    |                                            --- ^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    |                                            required by a bound introduced by this call
    |
note: required by a bound in `std::collections::BTreeMap::<K, V>::get`
   --> /checkout/library/alloc/src/collections/btree/map.rs:558:24
   --> /checkout/library/alloc/src/collections/btree/map.rs:558:24
    |
558 |         K: Borrow<Q> + Ord,
    |                        ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::get`

error[E0277]: the trait bound `BoundRegion: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/fold.rs:754:53
     |
754  |             |br: ty::BoundRegion| *region_map.entry(br).or_insert_with(|| fld_r(br));
     |                                               ----- ^^ the trait `Ord` is not implemented for `BoundRegion`
     |                                               required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_insert_with` exists for enum `std::collections::btree_map::Entry<'_, BoundRegion, _>`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/ty/fold.rs:754:57
    |
754 |             |br: ty::BoundRegion| *region_map.entry(br).or_insert_with(|| fld_r(br));
    |                                                         ^^^^^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, BoundRegion, _>` due to unsatisfied trait bounds
   ::: compiler/rustc_middle/src/ty/sty.rs:63:1
    |
63  | pub struct BoundRegion {
63  | pub struct BoundRegion {
    | ---------------------- doesn't satisfy `BoundRegion: Ord`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `BoundRegion: Ord`
help: consider annotating `BoundRegion` with `#[derive(Ord)]`
    |
63  | #[derive(Ord)]


error[E0277]: the trait bound `BoundRegion: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/fold.rs:807:66
     |
807  |         let real_fld_r = |br: ty::BoundRegion| *region_map.entry(br).or_insert_with(|| fld_r(br));
     |                                                            ----- ^^ the trait `Ord` is not implemented for `BoundRegion`
     |                                                            required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_insert_with` exists for enum `std::collections::btree_map::Entry<'_, BoundRegion, _>`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/ty/fold.rs:807:70
    |
807 |         let real_fld_r = |br: ty::BoundRegion| *region_map.entry(br).or_insert_with(|| fld_r(br));
    |                                                                      ^^^^^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, BoundRegion, _>` due to unsatisfied trait bounds
   ::: compiler/rustc_middle/src/ty/sty.rs:63:1
    |
63  | pub struct BoundRegion {
63  | pub struct BoundRegion {
    | ---------------------- doesn't satisfy `BoundRegion: Ord`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `BoundRegion: Ord`
help: consider annotating `BoundRegion` with `#[derive(Ord)]`
    |
63  | #[derive(Ord)]


error[E0277]: the trait bound `Binder<'tcx, sty::TraitRef<'tcx>>: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:866:38
     |
866  |                         traits.entry(fn_once_trait_ref).or_default().extend(
     |                                ----- ^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `Binder<'tcx, sty::TraitRef<'tcx>>`
     |                                required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_default` exists for enum `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:866:57
     |
866  |                         traits.entry(fn_once_trait_ref).or_default().extend(
     |                                                         ^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:1025:1
     |
     |
1025 | pub struct Binder<'tcx, T>(T, &'tcx List<BoundVariableKind>);
     | ------------------------------------------------------------- doesn't satisfy `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
help: consider annotating `Binder<'tcx, sty::TraitRef<'tcx>>` with `#[derive(Ord)]`
     |
1025 | #[derive(Ord)]


error[E0277]: the trait bound `Binder<'tcx, sty::TraitRef<'tcx>>: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:874:38
     |
874  |                         traits.entry(trait_ref).or_default();
     |                                ----- ^^^^^^^^^ the trait `Ord` is not implemented for `Binder<'tcx, sty::TraitRef<'tcx>>`
     |                                required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_default` exists for enum `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:874:49
     |
874  |                         traits.entry(trait_ref).or_default();
     |                                                 ^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:1025:1
     |
     |
1025 | pub struct Binder<'tcx, T>(T, &'tcx List<BoundVariableKind>);
     | ------------------------------------------------------------- doesn't satisfy `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
help: consider annotating `Binder<'tcx, sty::TraitRef<'tcx>>` with `#[derive(Ord)]`
     |
1025 | #[derive(Ord)]


error[E0277]: the trait bound `Binder<'tcx, sty::TraitRef<'tcx>>: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:877:38
     |
877  |                         traits.entry(trait_ref).or_default();
     |                                ----- ^^^^^^^^^ the trait `Ord` is not implemented for `Binder<'tcx, sty::TraitRef<'tcx>>`
     |                                required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_default` exists for enum `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:877:49
     |
877  |                         traits.entry(trait_ref).or_default();
     |                                                 ^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:1025:1
     |
     |
1025 | pub struct Binder<'tcx, T>(T, &'tcx List<BoundVariableKind>);
     | ------------------------------------------------------------- doesn't satisfy `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
help: consider annotating `Binder<'tcx, sty::TraitRef<'tcx>>` with `#[derive(Ord)]`
     |
1025 | #[derive(Ord)]


error[E0277]: the trait bound `Binder<'tcx, sty::TraitRef<'tcx>>: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:959:45
     |
959  |                 let entry = fn_traits.entry(trait_ref).or_default();
     |                                       ----- ^^^^^^^^^ the trait `Ord` is not implemented for `Binder<'tcx, sty::TraitRef<'tcx>>`
     |                                       required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_default` exists for enum `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, OpaqueFnEntry<'tcx>>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:959:56
     |
959  |                 let entry = fn_traits.entry(trait_ref).or_default();
     |                                                        ^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, OpaqueFnEntry<'tcx>>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:1025:1
     |
     |
1025 | pub struct Binder<'tcx, T>(T, &'tcx List<BoundVariableKind>);
     | ------------------------------------------------------------- doesn't satisfy `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
help: consider annotating `Binder<'tcx, sty::TraitRef<'tcx>>` with `#[derive(Ord)]`
     |
1025 | #[derive(Ord)]


error[E0277]: the trait bound `Binder<'tcx, sty::TraitRef<'tcx>>: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:971:33
     |
971  |                 fn_traits.entry(super_trait_ref).or_default().fn_mut_trait_ref = Some(trait_ref);
     |                           ----- ^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `Binder<'tcx, sty::TraitRef<'tcx>>`
     |                           required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_default` exists for enum `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, OpaqueFnEntry<'tcx>>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:971:50
     |
971  |                 fn_traits.entry(super_trait_ref).or_default().fn_mut_trait_ref = Some(trait_ref);
     |                                                  ^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, OpaqueFnEntry<'tcx>>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:1025:1
     |
     |
1025 | pub struct Binder<'tcx, T>(T, &'tcx List<BoundVariableKind>);
     | ------------------------------------------------------------- doesn't satisfy `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
help: consider annotating `Binder<'tcx, sty::TraitRef<'tcx>>` with `#[derive(Ord)]`
     |
1025 | #[derive(Ord)]


error[E0277]: the trait bound `Binder<'tcx, sty::TraitRef<'tcx>>: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:978:33
     |
978  |                 fn_traits.entry(super_trait_ref).or_default().fn_trait_ref = Some(trait_ref);
     |                           ----- ^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `Binder<'tcx, sty::TraitRef<'tcx>>`
     |                           required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_default` exists for enum `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, OpaqueFnEntry<'tcx>>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:978:50
     |
978  |                 fn_traits.entry(super_trait_ref).or_default().fn_trait_ref = Some(trait_ref);
     |                                                  ^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, OpaqueFnEntry<'tcx>>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:1025:1
     |
     |
1025 | pub struct Binder<'tcx, T>(T, &'tcx List<BoundVariableKind>);
     | ------------------------------------------------------------- doesn't satisfy `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
help: consider annotating `Binder<'tcx, sty::TraitRef<'tcx>>` with `#[derive(Ord)]`
     |
1025 | #[derive(Ord)]


error[E0277]: the trait bound `Binder<'tcx, sty::TraitRef<'tcx>>: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:984:22
     |
984  |         traits.entry(trait_ref).or_default().extend(proj_ty);
     |                ----- ^^^^^^^^^ the trait `Ord` is not implemented for `Binder<'tcx, sty::TraitRef<'tcx>>`
     |                required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_default` exists for enum `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:984:33
     |
984  |         traits.entry(trait_ref).or_default().extend(proj_ty);
     |                                 ^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, Binder<'tcx, sty::TraitRef<'tcx>>, std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:1025:1
     |
     |
1025 | pub struct Binder<'tcx, T>(T, &'tcx List<BoundVariableKind>);
     | ------------------------------------------------------------- doesn't satisfy `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `Binder<'tcx, sty::TraitRef<'tcx>>: Ord`
help: consider annotating `Binder<'tcx, sty::TraitRef<'tcx>>` with `#[derive(Ord)]`
     |
1025 | #[derive(Ord)]


error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:1089:21
1089 |         auto_traits.sort();
1089 |         auto_traits.sort();
     |                     ^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
     |
     = note: required because of the requirements on the impl of `Ord` for `(std::string::String, rustc_span::def_id::DefId)`
note: required by a bound in `std::slice::<impl [T]>::sort`
     |
     |
274  |         T: Ord,
     |            ^^^ required by this bound in `std::slice::<impl [T]>::sort`

error[E0277]: the trait bound `BoundRegion: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:2047:61
     |
2047 |             ty::ReLateBound(_, br) => self.region_map.entry(br).or_insert_with(|| name(br)),
     |                                                       ----- ^^ the trait `Ord` is not implemented for `BoundRegion`
     |                                                       required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_insert_with` exists for enum `std::collections::btree_map::Entry<'_, BoundRegion, &'tcx RegionKind>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:2047:65
     |
2047 |             ty::ReLateBound(_, br) => self.region_map.entry(br).or_insert_with(|| name(br)),
     |                                                                 ^^^^^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, BoundRegion, &'tcx RegionKind>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:63:1
     |
63   | pub struct BoundRegion {
63   | pub struct BoundRegion {
     | ---------------------- doesn't satisfy `BoundRegion: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `BoundRegion: Ord`
help: consider annotating `BoundRegion` with `#[derive(Ord)]`
     |
63   | #[derive(Ord)]


error[E0277]: the trait bound `BoundRegion: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:2056:47
     |
2056 |                         self.region_map.entry(br).or_insert_with(|| name(br))
     |                                         ----- ^^ the trait `Ord` is not implemented for `BoundRegion`
     |                                         required by a bound introduced by this call
     |
     |
note: required by a bound in `std::collections::BTreeMap::<K, V>::entry`
     |
     |
1140 |         K: Ord,
     |            ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::entry`

error[E0599]: the method `or_insert_with` exists for enum `std::collections::btree_map::Entry<'_, BoundRegion, &'tcx RegionKind>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:2056:51
     |
2056 |                         self.region_map.entry(br).or_insert_with(|| name(br))
     |                                                   ^^^^^^^^^^^^^^ method cannot be called on `std::collections::btree_map::Entry<'_, BoundRegion, &'tcx RegionKind>` due to unsatisfied trait bounds
    ::: compiler/rustc_middle/src/ty/sty.rs:63:1
     |
63   | pub struct BoundRegion {
63   | pub struct BoundRegion {
     | ---------------------- doesn't satisfy `BoundRegion: Ord`
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `BoundRegion: Ord`
help: consider annotating `BoundRegion` with `#[derive(Ord)]`
     |
63   | #[derive(Ord)]


error[E0599]: the method `cmp` exists for enum `GenericArgKind<'_>`, but its trait bounds were not satisfied
    --> compiler/rustc_middle/src/ty/subst.rs:82:23
     |
40   | pub enum GenericArgKind<'tcx> {
     | |
     | |
     | method `cmp` not found for this
     | doesn't satisfy `GenericArgKind<'_>: Iterator`
...
82   |         self.unpack().cmp(&other.unpack())
     |                       ^^^ method cannot be called on `GenericArgKind<'_>` due to unsatisfied trait bounds
     = note: the following trait bounds were not satisfied:
     = note: the following trait bounds were not satisfied:
             `GenericArgKind<'_>: Iterator`
             which is required by `&mut GenericArgKind<'_>: Iterator`
    --> /checkout/library/core/src/iter/traits/iterator.rs:55:1
     |
55   | / pub trait Iterator {
56   | |     /// The type of the elements being iterated over.
56   | |     /// The type of the elements being iterated over.
57   | |     #[stable(feature = "rust1", since = "1.0.0")]
58   | |     type Item;
3558 | |     }
3559 | | }
     | |_^
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
     = note: the following trait defines an item `cmp`, perhaps you need to implement it:
             candidate #1: `Iterator`

error[E0599]: the method `cmp` exists for struct `rustc_span::def_id::DefId`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/ty/adt.rs:112:18
    |
112 |         self.did.cmp(&other.did)
    |                  ^^^ method cannot be called on `rustc_span::def_id::DefId` due to unsatisfied trait bounds
   ::: /checkout/compiler/rustc_span/src/def_id.rs:225:1
    |
225 | pub struct DefId {
225 | pub struct DefId {
    | ---------------- doesn't satisfy `rustc_span::def_id::DefId: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `rustc_span::def_id::DefId: Iterator`
            which is required by `&mut rustc_span::def_id::DefId: Iterator`

error[E0277]: can't compare `TyS<'_>` with `TyS<'_>`
     |
     |
1762 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TyEncodable, HashStable)]
     |                                        ---------- in this derive macro expansion
...
1767 |       pub projection: &'tcx List<PlaceElem<'tcx>>,
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `TyS<'_> < TyS<'_>` and `TyS<'_> > TyS<'_>`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `TyS<'_>`
note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `&TyS<'_>`
    --> compiler/rustc_middle/src/ty/mod.rs:427:7
     |
427  | impl !PartialOrd for TyS<'_> {}
     |       ^^^^^^^^^^     ^^^^^^^
     = note: 3 redundant requirements hidden
     = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&list::List<mir::ProjectionElem<mir::Local, &TyS<'_>>>>` for `&list::List<mir::ProjectionElem<mir::Local, &TyS<'_>>>`

error[E0277]: the trait bound `TyS<'_>: Ord` is not satisfied
     |
     |
1762 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, TyEncodable, HashStable)]
     |                                                    --- in this derive macro expansion
...
1767 |       pub projection: &'tcx List<PlaceElem<'tcx>>,
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `TyS<'_>`
    ::: /checkout/library/core/src/cmp.rs:832:1
     |
     |
832  | / pub macro Ord($item:item) {
834  | | }
834  | | }
     | |_- in this expansion of `#[derive(Ord)]`
     |
note: required because of the requirements on the impl of `Ord` for `&TyS<'_>`
    --> compiler/rustc_middle/src/ty/mod.rs:426:7
     |
426  | impl !Ord for TyS<'_> {}
     |       ^^^     ^^^^^^^
     = note: 3 redundant requirements hidden
     = note: required because of the requirements on the impl of `Ord` for `&list::List<mir::ProjectionElem<mir::Local, &TyS<'_>>>`

error[E0277]: can't compare `TyS<'_>` with `TyS<'_>`
     |
     |
1866 |   #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
     |                                               ---------- in this derive macro expansion
...
1869 |       pub projection: &'tcx [PlaceElem<'tcx>],
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `TyS<'_> < TyS<'_>` and `TyS<'_> > TyS<'_>`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `TyS<'_>`
note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `&TyS<'_>`
    --> compiler/rustc_middle/src/ty/mod.rs:427:7
     |
427  | impl !PartialOrd for TyS<'_> {}
     |       ^^^^^^^^^^     ^^^^^^^
     = note: 3 redundant requirements hidden
