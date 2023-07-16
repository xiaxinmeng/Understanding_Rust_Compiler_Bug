plain
[00:05:42]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:46]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:18]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:38]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:39] error[E0658]: :lifetime fragment specifier is experimental and subject to change (see issue #46895)
[00:07:39]    --> librustc/macros.rs:123:11
[00:07:39]     |
[00:07:39] 123 |     (impl<$tcx:lifetime $(, $T:ident)*> for struct $struct_name:path {
[00:07:39]     |
[00:07:39]     |
[00:07:39]     = help: add #![feature(macro_lifetime_matcher)] to the crate attributes to enable
[00:07:39] 
[00:07:39] error[E0658]: :lifetime fragment specifier is experimental and subject to change (see issue #46895)
[00:07:39]    --> librustc/macros.rs:168:11
[00:07:39]     |
[00:07:39] 168 |     (for <$tcx:lifetime> { $($ty:ty,)+ }) => {
[00:07:39]     |
[00:07:39]     |
[00:07:39]     = help: add #![feature(macro_lifetime_matcher)] to the crate attributes to enable
[00:07:39] 
[00:07:39] error[E0658]: :lifetime fragment specifier is experimental and subject to change (see issue #46895)
[00:07:39]    --> librustc/macros.rs:192:11
[00:07:39]     |
[00:07:39] 192 |     (for <$tcx:lifetime> { $($ty:ty,)+ }) => {
[00:07:39]     |
[00:07:39]     |
[00:07:39]     = help: add #![feature(macro_lifetime_matcher)] to the crate attributes to enable
[00:07:39] 
[00:07:42] error: unused import: `middle`
[00:07:42]   --> librustc/ich/impls_cstore.rs:14:5
[00:07:42] 14 | use middle;
[00:07:42]    |     ^^^^^^
[00:07:42]    |
[00:07:42]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:42]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:42] 
[00:07:42] error: unused import: `infer`
[00:07:42]   --> librustc/ich/impls_ty.rs:22:5
[00:07:42] 22 | use infer;
[00:07:42]    |     ^^^^^
[00:07:42] 
[00:07:42] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ich/hcx.rs:266:10
[00:07:46]     |
[00:07:46] 266 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for hir::HirId {
[00:07:46]     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, hir::ItemLocalId)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::ItemLocalId: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ich/hcx.rs:266:10
[00:07:46]     |
[00:07:46] 266 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for hir::HirId {
[00:07:46]     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::ItemLocalId`
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, hir::ItemLocalId)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ich/hcx.rs:293:10
[00:07:46]     |
[00:07:46] 293 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for ast::NodeId {
[00:07:46]     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, hir::ItemLocalId)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::ItemLocalId: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ich/hcx.rs:293:10
[00:07:46]     |
[00:07:46] 293 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for ast::NodeId {
[00:07:46]     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::ItemLocalId`
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, hir::ItemLocalId)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]   --> librustc/ich/impls_hir.rs:33:10
[00:07:46]    |
[00:07:46] 33 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for DefId {
[00:07:46]    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]   --> librustc/ich/impls_hir.rs:51:10
[00:07:46]    |
[00:07:46] 51 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for LocalDefId {
[00:07:46]    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]   --> librustc/ich/impls_hir.rs:72:10
[00:07:46]    |
[00:07:46] 72 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for CrateNum {
[00:07:46]    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::ItemLocalId: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]   --> librustc/ich/impls_hir.rs:84:10
[00:07:46]    |
[00:07:46] 84 | impl<'a> ToStableHashKey<StableHashingContext<'a>>
[00:07:46]    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::ItemLocalId`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ich/impls_hir.rs:977:10
[00:07:46]     |
[00:07:46] 977 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for hir::BodyId {
[00:07:46]     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, hir::ItemLocalId)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::ItemLocalId: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ich/impls_hir.rs:977:10
[00:07:46]     |
[00:07:46] 977 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for hir::BodyId {
[00:07:46]     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::ItemLocalId`
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, hir::ItemLocalId)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]     --> librustc/ich/impls_hir.rs:1101:10
[00:07:46]      |
[00:07:46] 1101 | impl<'a> ToStableHashKey<StableHashingContext<'a>>
[00:07:46]      |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::map::definitions::DefPathHash: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]     --> librustc/ich/impls_hir.rs:1150:10
[00:07:46]      |
[00:07:46] 1150 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for hir::TraitCandidate {
[00:07:46]      |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::map::definitions::DefPathHash`
[00:07:46]      |
[00:07:46]      = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, std::option::Option<(hir::map::definitions::DefPathHash, hir::ItemLocalId)>)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::ItemLocalId: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]     --> librustc/ich/impls_hir.rs:1150:10
[00:07:46]      |
[00:07:46] 1150 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for hir::TraitCandidate {
[00:07:46]      |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::ItemLocalId`
[00:07:46]      |
[00:07:46]      = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, hir::ItemLocalId)`
[00:07:46]      = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::option::Option<(hir::map::definitions::DefPathHash, hir::ItemLocalId)>`
[00:07:46]      = note: required because of the requirements on the impl of `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `(hir::map::definitions::DefPathHash, std::option::Option<(hir::map::definitions::DefPathHash, hir::ItemLocalId)>)`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `middle::region::Scope: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ich/impls_ty.rs:822:10
[00:07:46]     |
[00:07:46] 822 | impl<'a> ToStableHashKey<StableHashingContext<'a>> for region::Scope {
[00:07:46]     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `middle::region::Scope`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `session::config::OutputType: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/session/config.rs:129:16
[00:07:46]     |
[00:07:46] 129 | impl<'a, 'tcx> ToStableHashKey<StableHashingContext<'a>> for OutputType {
[00:07:46]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `session::config::OutputType`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `hir::Unsafety: ty::fold::TypeFoldable<'tcx>` is not satisfied
[00:07:46]    --> librustc/ty/relate.rs:188:12
[00:07:46]     |
[00:07:46] 188 | impl<'tcx> Relate<'tcx> for ast::Unsafety {
[00:07:46]     |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `hir::Unsafety`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `rustc_target::spec::abi::Abi: ty::fold::TypeFoldable<'tcx>` is not satisfied
[00:07:46]    --> librustc/ty/relate.rs:203:12
[00:07:46]     |
[00:07:46] 203 | impl<'tcx> Relate<'tcx> for abi::Abi {
[00:07:46]     |            ^^^^^^^^^^^^ the trait `ty::fold::TypeFoldable<'tcx>` is not implemented for `rustc_target::spec::abi::Abi`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::GenericPredicates<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::GenericPredicates<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::Destructor: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::Destructor`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::option::Option<ty::Destructor>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> traits::query::dropck_outlives::DtorckConstraint<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `traits::query::dropck_outlives::DtorckConstraint<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::result::Result<traits::query::dropck_outlives::DtorckConstraint<'_>, traits::query::NoSolution>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> traits::query::NoSolution: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `traits::query::NoSolution`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::result::Result<traits::query::dropck_outlives::DtorckConstraint<'_>, traits::query::NoSolution>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::Variance: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::Variance`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::vec::Vec<ty::Variance>`
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::rc::Rc<std::vec::Vec<ty::Variance>>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::AssociatedItem: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::AssociatedItem`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::sty::TraitRef<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::sty::TraitRef<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::option::Option<ty::sty::TraitRef<'_>>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> hir::ImplPolarity: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `hir::ImplPolarity`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> mir::Mir<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `mir::Mir<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `ty::steal::Steal<mir::Mir<'_>>`
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `&ty::steal::Steal<mir::Mir<'_>>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> mir::Mir<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `mir::Mir<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `&mir::Mir<'_>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> mir::UnsafetyCheckResult: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `mir::UnsafetyCheckResult`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::sty::FnSig<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::sty::FnSig<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `ty::sty::Binder<ty::sty::FnSig<'_>>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::adjustment::CoerceUnsizedInfo: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::adjustment::CoerceUnsizedInfo`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> session::CompileIncomplete: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `session::CompileIncomplete`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::result::Result<(), session::CompileIncomplete>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> mir::BorrowCheckResult<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `mir::BorrowCheckResult<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::CrateInherentImpls: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::CrateInherentImpls`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> middle::const_val::ConstEvalErr<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `middle::const_val::ConstEvalErr<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::result::Result<&ty::sty::Const<'_>, middle::const_val::ConstEvalErr<'_>>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> ty::sty::Const<'_>: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `ty::sty::Const<'_>`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
[00:07:46]     |
[00:07:46] 95  | / define_maps! { <'tcx>
[00:07:46] 96  | |     /// Records the type of every item.
[00:07:46] 97  | |     [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:07:46] ...   |
[00:07:46] ...   |
[00:07:46] 461 | |         -> Lrc<FxHashMap<DefId, String>>,
[00:07:46] 462 | | }
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `&ty::sty::Const<'_>`
[00:07:46]     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` for `std::result::Result<&ty::sty::Const<'_>, middle::const_val::ConstEvalErr<'_>>`
[00:07:46] 
[00:07:46] error[E0277]: the trait bound `for<'a> util::common::ErrorReported: rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not satisfied
[00:07:46]    --> librustc/ty/maps/plumbing.rs:700:22
[00:07:46]     |
[00:07:46] 700 |           $(impl<$tcx> QueryConfig<$tcx> for queries::$name<$tcx> {
[00:07:46]     |                        ^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<ich::hcx::StableHashingContext<'a>>` is not implemented for `util::common::ErrorReported`
[00:07:46]     | 
[00:07:46]    ::: librustc/ty/maps/mod.rs:95:1
---
12160 ./src/llvm-emscripten/include/llvm
11900 ./src/tools/lld
11740 ./src/doc
10052 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
travis_time:end:1fb798e4:start=1525236537986986352,finish=1525236538185639211,duration=198652859
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:00b7fde8
