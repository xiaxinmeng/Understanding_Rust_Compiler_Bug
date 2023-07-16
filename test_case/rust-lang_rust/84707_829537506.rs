plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `crate::clean::MAX_DEF_IDX`
  --> src/librustdoc/core.rs:35:40
   |
35 | use crate::clean::{TraitWithExtraInfo, MAX_DEF_IDX};
   |                                        ^^^^^^^^^^^ no `MAX_DEF_IDX` in `clean`

error[E0412]: cannot find type `LocalDefId` in this scope
  --> src/librustdoc/clean/types.rs:85:39
   |
53 | crate type RealDefId = rustc_hir::def_id::DefId;
   | ------------------------------------------------ similarly named type alias `RealDefId` defined here
...
85 |     crate fn as_local(self) -> Option<LocalDefId> {
   |
help: a type alias with a similar name exists
   |
   |
85 |     crate fn as_local(self) -> Option<RealDefId> {
help: consider importing one of these items
   |
   |
1  | use crate::passes::collect_intra_doc_links::early::LocalDefId;
1  | use rustc_hir::def_id::LocalDefId;
   |
1  | use rustc_span::def_id::LocalDefId;
   |
   |

error[E0412]: cannot find type `LocalDefId` in this scope
  --> src/librustdoc/clean/types.rs:93:36
   |
53 | crate type RealDefId = rustc_hir::def_id::DefId;
   | ------------------------------------------------ similarly named type alias `RealDefId` defined here
...
93 |     crate fn expect_local(self) -> LocalDefId {
   |
help: a type alias with a similar name exists
   |
   |
93 |     crate fn expect_local(self) -> RealDefId {
help: consider importing one of these items
   |
   |
1  | use crate::passes::collect_intra_doc_links::early::LocalDefId;
1  | use rustc_hir::def_id::LocalDefId;
   |
1  | use rustc_span::def_id::LocalDefId;
   |
   |

error[E0574]: expected struct, variant or union type, found enum `DefId`
   --> src/librustdoc/clean/types.rs:157:9
    |
157 |         DefId { krate: self.crate_num, index: CRATE_DEF_INDEX }
    |         ^^^^^ not a struct, variant or union type
help: consider importing one of these items instead
    |
    |
1   | use crate::core::ImplTraitParam::DefId;
1   | use rustc_hir::def_id::DefId;
    |
1   | use rustc_span::def_id::DefId;
    |
    |

error[E0574]: expected struct, variant or union type, found enum `DefId`
    --> src/librustdoc/clean/mod.rs:1974:24
     |
1974 |     let crate_def_id = DefId { krate: cnum, index: CRATE_DEF_INDEX };
     |                        ^^^^^ not a struct, variant or union type
help: consider importing one of these items instead
     |
     |
12   | use crate::core::ImplTraitParam::DefId;
12   | use rustc_hir::def_id::DefId;
     |
12   | use rustc_span::def_id::DefId;
     |
     |

error[E0574]: expected struct, variant or union type, found enum `DefId`
    |
    |
167 |         DefId { krate: crate_num, index: *def_index }
    |         ^^^^^ not a struct, variant or union type
help: consider importing one of these items instead
    |
    |
1   | use crate::core::ImplTraitParam::DefId;
1   | use rustc_hir::def_id::DefId;
    |
1   | use rustc_span::def_id::DefId;
    |
    |

error[E0425]: cannot find value `f_id` in this scope
    |
    |
175 |                 Some(real_id.as_local().map(|def_id| tcx.hir().local_def_id_to_hir_id(f_id)))

error: unused import: `std::collections::HashSet`
  --> src/librustdoc/clean/types.rs:10:5
   |
   |
10 | use std::collections::HashSet;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `DefIndex`
  --> src/librustdoc/clean/types.rs:22:35
   |
   |
22 | use rustc_hir::def_id::{CrateNum, DefIndex, CRATE_DEF_INDEX};


error: Prefer FxHashSet over HashSet, it has better performance
  --> src/librustdoc/clean/types.rs:10:23
10 | use std::collections::HashSet;
10 | use std::collections::HashSet;
   |                       ^^^^^^^ help: use: `FxHashSet`
   |
   = note: `-D rustc::default-hash-types` implied by `-D warnings`
   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 48 elements, found one with 64 elements
7   | | }
    | |_- in this expansion of `rustc_data_structures::static_assert_size!`
    | 
   ::: src/librustdoc/clean/types.rs:304:1
   ::: src/librustdoc/clean/types.rs:304:1
    |
304 |   rustc_data_structures::static_assert_size!(Item, 48);

error[E0308]: mismatched types
    --> src/librustdoc/html/format.rs:1154:74
     |
     |
1154 |                 let parent_module = find_nearest_parent_module(cx.tcx(), item_did);
     |                                                                          |
     |                                                                          |
     |                                                                          expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                                          help: try using a variant of the expected enum: `types::DefId::Real(item_did)`

error[E0609]: no field `index` on type `types::DefId`
    --> src/librustdoc/html/format.rs:1156:28
     |
1156 |                 if vis_did.index == CRATE_DEF_INDEX {

error[E0308]: mismatched types
    --> src/librustdoc/html/format.rs:1169:50
     |
     |
1169 |                     let path = cx.tcx().def_path(vis_did);
     |                                                  ^^^^^^^ expected struct `rustc_span::def_id::DefId`, found enum `types::DefId`
error[E0308]: mismatched types
    --> src/librustdoc/html/format.rs:1173:41
     |
     |
1173 |                     let anchor = anchor(vis_did, &last_name.as_str(), cx).to_string();
     |                                         ^^^^^^^ expected struct `rustc_span::def_id::DefId`, found enum `types::DefId`
error[E0308]: mismatched types
    --> src/librustdoc/html/format.rs:1202:69
     |
     |
1202 |                 let parent_module = find_nearest_parent_module(tcx, item_did);
     |                                                                     |
     |                                                                     |
     |                                                                     expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                                     help: try using a variant of the expected enum: `types::DefId::Real(item_did)`

error[E0609]: no field `index` on type `types::DefId`
    --> src/librustdoc/html/format.rs:1204:28
     |
1204 |                 if vis_did.index == CRATE_DEF_INDEX {

error[E0308]: mismatched types
    --> src/librustdoc/html/format.rs:1217:61
     |
     |
1217 |                     format!("pub(in {}) ", tcx.def_path_str(vis_did))
     |                                                             ^^^^^^^ expected struct `rustc_span::def_id::DefId`, found enum `types::DefId`
error[E0308]: mismatched types
    --> src/librustdoc/html/format.rs:1295:43
     |
     |
1295 |             Some(did) => resolved_path(f, did, &self.path, true, false, cx),
     |                                           ^^^ expected struct `rustc_span::def_id::DefId`, found enum `types::DefId`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:164:18
    |
    |
164 |             Some(trait_ref.def_id),
    |                  |
    |                  |
    |                  expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                  help: try using a variant of the expected enum: `types::DefId::Real(trait_ref.def_id)`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:172:54
    |
    |
172 |         ResolvedPath { path, param_names: None, did: trait_ref.def_id, is_generic: false }
    |                                                      |
    |                                                      |
    |                                                      expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                                                      help: try using a variant of the expected enum: `types::DefId::Real(trait_ref.def_id)`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:242:52
    |
    |
242 |                 if let Some(lt) = cx.lt_substs.get(&node_id).cloned() {
    |                                                    ^^^^^^^^ expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |
    = note: expected reference `&types::DefId`
               found reference `&rustc_span::def_id::DefId`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:430:30
    |
430 |                         did: self.def_id,
430 |                         did: self.def_id,
    |                              ^^^^^^^^^^^
    |                              |
    |                              expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                              help: try using a variant of the expected enum: `types::DefId::Real(self.def_id)`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:440:26
    |
440 |                     did: self.def_id,
440 |                     did: self.def_id,
    |                          ^^^^^^^^^^^
    |                          |
    |                          expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                          help: try using a variant of the expected enum: `types::DefId::Real(self.def_id)`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:473:26
    |
    |
473 |                     did: cx.tcx.hir().local_def_id(self.hir_id).to_def_id(),
    |                          |
    |                          |
    |                          expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                          help: try using a variant of the expected enum: `types::DefId::Real(cx.tcx.hir().local_def_id(self.hir_id).to_def_id())`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:482:26
    |
    |
482 |                     did: cx.tcx.hir().local_def_id(self.hir_id).to_def_id(),
    |                          |
    |                          |
    |                          expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                          help: try using a variant of the expected enum: `types::DefId::Real(cx.tcx.hir().local_def_id(self.hir_id).to_def_id())`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:679:65
    |
    |
679 |                         simplify::merge_bounds(cx, &mut bounds, trait_did, name, &rhs);
    |                                                                 ^^^^^^^^^ expected struct `rustc_span::def_id::DefId`, found enum `types::DefId`

error[E0277]: the trait bound `types::DefId: rustc_middle::ty::query::sealed::IntoQueryParam<rustc_span::def_id::DefId>` is not satisfied
   --> src/librustdoc/clean/mod.rs:859:78
    |
859 |         let mut names = if did.is_local() { &[] } else { cx.tcx.fn_arg_names(did) }.iter();
    |                                                                              ^^^ the trait `rustc_middle::ty::query::sealed::IntoQueryParam<rustc_span::def_id::DefId>` is not implemented for `types::DefId`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:916:27
    |
    |
916 |         cx.with_param_env(local_did, |cx| {
    |                           |
    |                           |
    |                           expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                           help: try using a variant of the expected enum: `types::DefId::Real(local_did)`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:947:45
    |
    |
947 |                 Item::from_def_id_and_parts(local_did, Some(self.ident.name), inner, cx);
    |                                             |
    |                                             |
    |                                             expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                                             help: try using a variant of the expected enum: `types::DefId::Real(local_did)`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:957:27
    |
    |
957 |         cx.with_param_env(local_did, |cx| {
    |                           |
    |                           |
    |                           expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                           help: try using a variant of the expected enum: `types::DefId::Real(local_did)`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:986:45
    |
    |
986 |                 Item::from_def_id_and_parts(local_did, Some(self.ident.name), inner, cx);
    |                                             |
    |                                             |
    |                                             expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
    |                                             help: try using a variant of the expected enum: `types::DefId::Real(local_did)`

error[E0599]: no method named `clean` found for tuple `(rustc_span::def_id::DefId, Binder<'_, rustc_middle::ty::FnSig<'_>>)` in the current scope
    --> src/librustdoc/clean/mod.rs:1021:51
     |
1021 |                 let mut decl = (self.def_id, sig).clean(cx);
     |                                                   ^^^^^ method not found in `(rustc_span::def_id::DefId, Binder<'_, rustc_middle::ty::FnSig<'_>>)`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `clean::Clean` defines an item `clean`, perhaps you need to implement it
    --> src/librustdoc/clean/mod.rs:52:1
     |
52   | crate trait Clean<T> {

error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1106:68
     |
     |
1106 | ...                   ResolvedPath { did, .. } if did == self.container.id() => {}
     |                                                          |
     |                                                          |
     |                                                          expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                          help: try using a variant of the expected enum: `types::DefId::Real(self.container.id())`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1151:37
     |
     |
1151 |         Item::from_def_id_and_parts(self.def_id, Some(self.ident.name), kind, cx)
     |                                     |
     |                                     |
     |                                     expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                     help: try using a variant of the expected enum: `types::DefId::Real(self.def_id)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1166:56
     |
     |
1166 |                 if let Some(new_ty) = cx.ty_substs.get(&did).cloned() {
     |                                                        ^^^^ expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |
     = note: expected reference `&types::DefId`
                found reference `&rustc_span::def_id::DefId`

error[E0277]: the trait bound `ImplTraitParam: From<rustc_span::def_id::DefId>` is not satisfied
    --> src/librustdoc/clean/mod.rs:1169:72
     |
1169 |                 if let Some(bounds) = cx.impl_trait_bounds.remove(&did.into()) {
     |                                                                        ^^^^ the trait `From<rustc_span::def_id::DefId>` is not implemented for `ImplTraitParam`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <ImplTraitParam as From<types::DefId>>
               <ImplTraitParam as From<u32>>
     = note: required because of the requirements on the impl of `Into<ImplTraitParam>` for `rustc_span::def_id::DefId`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1261:39
     |
     |
1261 |                 return cx.enter_alias(ty_substs, lt_substs, ct_substs, |cx| ty.clean(cx));
     |                                       ^^^^^^^^^ expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |
     = note: expected struct `HashMap<types::DefId, _, _>`
                found struct `HashMap<rustc_span::def_id::DefId, _, _>`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1261:50
     |
     |
1261 |                 return cx.enter_alias(ty_substs, lt_substs, ct_substs, |cx| ty.clean(cx));
     |                                                  ^^^^^^^^^ expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |
     = note: expected struct `HashMap<types::DefId, _, _>`
                found struct `HashMap<rustc_span::def_id::DefId, _, _>`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1261:61
     |
     |
1261 |                 return cx.enter_alias(ty_substs, lt_substs, ct_substs, |cx| ty.clean(cx));
     |                                                             ^^^^^^^^^ expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |
     = note: expected struct `HashMap<types::DefId, _, _>`
                found struct `HashMap<rustc_span::def_id::DefId, _, _>`

error[E0599]: no variant or associated item named `local` found for enum `types::DefId` in the current scope
    --> src/librustdoc/clean/mod.rs:1436:37
     |
1436 |                 let def_id = DefId::local(CRATE_DEF_INDEX);
     |                                     ^^^^^ variant or associated item not found in `types::DefId`
    ::: src/librustdoc/clean/types.rs:56:1
     |
     |
56   | crate enum DefId {
     | ---------------- variant or associated item `local` not found here
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1453:57
     |
     |
1453 |                 ResolvedPath { path, param_names: None, did, is_generic: false }
     |                                                         |
     |                                                         |
     |                                                         expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                         help: try using a variant of the expected enum: `types::DefId::Real(did)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1465:57
     |
     |
1465 |                 ResolvedPath { path, param_names: None, did, is_generic: false }
     |                                                         |
     |                                                         |
     |                                                         expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                         help: try using a variant of the expected enum: `types::DefId::Real(did)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1490:71
     |
     |
1490 |                         external_path(cx, cx.tcx.item_name(did), Some(did), false, vec![], empty);
     |                                                                       |
     |                                                                       |
     |                                                                       expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                                       help: try using a variant of the expected enum: `types::DefId::Real(did)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1497:33
     |
1497 | ...                   did,
1497 | ...                   did,
     |                       ^^^
     |                       |
     |                       expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                       help: try using a variant of the expected enum: `types::DefId::Real(did)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1516:67
     |
     |
1516 |                     external_path(cx, cx.tcx.item_name(did), Some(did), false, bindings, substs);
     |                                                                   |
     |                                                                   |
     |                                                                   expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                                   help: try using a variant of the expected enum: `types::DefId::Real(did)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1517:70
     |
     |
1517 |                 ResolvedPath { path, param_names: Some(param_names), did, is_generic: false }
     |                                                                      |
     |                                                                      |
     |                                                                      expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                                      help: try using a variant of the expected enum: `types::DefId::Real(did)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1644:13
     |
1644 |             self.did,
1644 |             self.did,
     |             ^^^^^^^^
     |             |
     |             expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |             help: try using a variant of the expected enum: `types::DefId::Real(self.did)`

error[E0599]: no variant or associated item named `local` found for enum `types::DefId` in the current scope
    --> src/librustdoc/clean/mod.rs:1660:36
     |
1660 |                 let krate = DefId::local(CRATE_DEF_INDEX);
     |                                    ^^^^^ variant or associated item not found in `types::DefId`
    ::: src/librustdoc/clean/types.rs:56:1
     |
     |
56   | crate enum DefId {
     | ---------------- variant or associated item `local` not found here
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1682:74
     |
     |
1682 |             ty::Visibility::Restricted(module) => Visibility::Restricted(module),
     |                                                                          |
     |                                                                          |
     |                                                                          expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                                          help: try using a variant of the expected enum: `types::DefId::Real(module)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1714:57
     |
     |
1714 | ...                   Item::from_def_id_and_parts(field.did, name, kind, cx);
     |                                                   |
     |                                                   |
     |                                                   expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                                   help: try using a variant of the expected enum: `types::DefId::Real(field.did)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1722:41
     |
     |
1722 |             Item::from_def_id_and_parts(self.def_id, Some(self.ident.name), VariantItem(kind), cx);
     |                                         |
     |                                         |
     |                                         expected enum `types::DefId`, found struct `rustc_span::def_id::DefId`
     |                                         help: try using a variant of the expected enum: `types::DefId::Real(self.def_id)`
error[E0308]: mismatched types
    --> src/librustdoc/clean/mod.rs:1820:27
     |
     |
1820 |         cx.with_param_env(def_id, |cx| {
