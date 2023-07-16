plain
travis_time:end:1efb22e2:start=1560287642906833972,finish=1560287646733087967,duration=3826253995
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:54]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:54] error: this file contains an un-closed delimiter
[00:08:54]     --> src/librustc/middle/region.rs:1411:3
[00:08:54]      |
[00:08:54] 742  | ) {
[00:08:54]      |   - un-closed delimiter
[00:08:54] 743  |     match visitor.cx.var_parent {
[00:08:54]      |                                 - this delimiter might not be properly closed...
[00:08:54] 751  |   }
[00:08:54] 751  |   }
[00:08:54]      |   - ...as it matches this but it has different indentation
[00:08:54] 1411 | }
[00:08:54]      |   ^
[00:08:54] 
[00:08:54] error: incorrect close delimiter: `}`
[00:08:54] error: incorrect close delimiter: `}`
[00:08:54]    --> src/librustc/middle/region.rs:855:88
[00:08:54]     |
[00:08:54] 855 |     debug!("resolve_pat - post-increment {} pat = {:?}", visitor.expr_and_pat_count, pa}
[00:08:54]     |           - un-closed delimiter                                                        ^ incorrect close delimiter
[00:08:55] error: this file contains an un-closed delimiter
[00:08:55]     --> src/librustc/ty/mod.rs:3431:3
[00:08:55]      |
[00:08:55]      |
[00:08:55] 3200 | fn associated_item_def_ids<'tcx>(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> &'tcx [DefId] {
[00:08:55]      |                                                                                           - un-closed delimiter
[00:08:55] 3203 |     match item.node {
[00:08:55]      |                     - this delimiter might not be properly closed...
[00:08:55] ...
[00:08:55] 3220 |   }
[00:08:55] 3220 |   }
[00:08:55]      |   - ...as it matches this but it has different indentation
[00:08:55] 3431 | }
[00:08:55]      |   ^
[00:08:55] 
[00:08:55] error: this file contains an un-closed delimiter
---
[00:08:55]     | - un-closed delimiter
[00:08:55] 186 |     Ok(ty::GenericPredicates {
[00:08:55]     |       - un-closed delimiter
[00:08:55] ...
[00:08:55] 244 |     Ok(decoder.tcx()
[00:08:55]     |       - this delimiter might not be properly closed...
[00:08:55] 245 |               .mk_existential_predicates((0..len).map(|_| Decodable::decode(decoder)))?)
[00:08:55]     |                                                                                        - ...as it matches this but it has different indentation
[00:08:55] 453 | }
[00:08:55]     |   ^
[00:08:55] 
[00:08:55] error: incorrect close delimiter: `}`
[00:08:55] error: incorrect close delimiter: `}`
[00:08:55]    --> src/librustc/ty/codec.rs:211:66
[00:08:55]     |
[00:08:55] 211 |     Ok(tcx.mk_substs((0..len).map(|_| Decodable::decode(decoder))}
[00:08:55]     |                     - un-closed delimiter                        ^ incorrect close delimiter
[00:08:55] error: incorrect close delimiter: `}`
[00:08:55]    --> src/librustc/ty/codec.rs:218:58
[00:08:55]     |
[00:08:55]     |
[00:08:55] 218 |     Ok(decoder.tcx().mk_region(Decodable::decode(decoder)}
[00:08:55]     |                               - un-closed delimiter      ^ incorrect close delimiter
[00:08:55] error: incorrect close delimiter: `}`
[00:08:55]    --> src/librustc/ty/codec.rs:226:79
[00:08:55]     |
[00:08:55]     |
[00:08:55] 226 |     Ok(decoder.tcx().mk_type_list((0..len).map(|_| Decodable::decode(decoder))}
[00:08:55]     |                                  - un-closed delimiter                        ^ incorrect close delimiter
[00:08:55] error: incorrect close delimiter: `}`
[00:08:55]    --> src/librustc/ty/codec.rs:234:35
[00:08:55]     |
[00:08:55]     |
[00:08:55] 234 |     Ok(decoder.tcx().adt_def(def_i}
[00:08:55]     |                             |
[00:08:55]     |                             un-closed delimiter
[00:08:55] 
[00:08:55] 
[00:08:55] error: expected one of `)`, `,`, `.`, `?`, or an operator, found `#`
[00:08:55]    --> src/librustc/ty/codec.rs:204:1
[00:08:55] 202 |    }
[00:08:55] 202 |    }
[00:08:55]     |     - expected one of `)`, `,`, `.`, `?`, or an operator here
[00:08:55] 204 | #[inline]
[00:08:55]     | ^ unexpected token
[00:08:55] 
[00:08:55] 
[00:08:55] error: expected one of `)`, `-`, `.`, `;`, `<`, `?`, `async`, `await`, `break`, `continue`, `for`, `if`, `loop`, `match`, `move`, `return`, `static`, `unsafe`, `while`, `yield`, `}`, or an operator, found `pub`
[00:08:55]    --> src/librustc/ty/codec.rs:205:1
[00:08:55] 204 | #[inline]
[00:08:55]     |          - expected one of 22 possible tokens here
[00:08:55]     |          - expected one of 22 possible tokens here
[00:08:55] 205 | pub fn decode_substs<D>(decoder: &mut D) -> Result<SubstsRef<'tcx>, D::Error>
[00:08:55] 
[00:08:55] 
[00:08:57] error[E0432]: unresolved import `crate::ty::CrateInherentImpls`
[00:08:57]   --> src/librustc/ty/query/mod.rs:37:23
[00:08:57]    |
[00:08:57] 37 | use crate::ty::{self, CrateInherentImpls, ParamEnvAnd, Ty, TyCtxt, AdtSizedConstraint};
[00:08:57]    |                       ^^^^^^^^^^^^^^^^^^ no `CrateInherentImpls` in `ty`
[00:08:57] error[E0432]: unresolved import `crate::ty::SymbolName`
[00:08:57]  --> src/librustc/mir/mono.rs:6:48
[00:08:57]   |
[00:08:57]   |
[00:08:57] 6 | use crate::ty::{Instance, InstanceDef, TyCtxt, SymbolName, subst::InternalSubsts};
[00:08:57]   |                                                ^^^^^^^^^^ no `SymbolName` in `ty`
[00:08:57] 
[00:09:01] error: cannot find macro `implement_ty_decoder!` in this scope
[00:09:01]    --> src/librustc/ty/query/on_disk_cache.rs:591:1
[00:09:01]     |
[00:09:01] 591 | implement_ty_decoder!(CacheDecoder<'a, 'tcx>);
[00:09:01] 
[00:09:01] 
[00:09:03] error[E0412]: cannot find type `CrateInherentImpls` in module `rustc::ty`
[00:09:03]     |
[00:09:03]     |
[00:09:03] 19  | / macro_rules! arena_types {
[00:09:03] 20  | |     ($macro:path, $args:tt, $tcx:lifetime) => (
[00:09:03] 21  | |         $macro!($args, [
[00:09:03] 22  | |             [] layouts: rustc::ty::layout::LayoutDetails,
[00:09:03] ...   |
[00:09:03] 78  | |             [few] crate_inherent_impls: rustc::ty::CrateInherentImpls,
[00:09:03] ...   |
[00:09:03] 114 | |     )
[00:09:03] 115 | | }
[00:09:03] 115 | | }
[00:09:03]     | |_- in this expansion of `arena_types!`
[00:09:03] ...
[00:09:03] 166 |   arena_types!(declare_arena, [], 'tcx);
[00:09:03] 
[00:09:03] 
[00:09:03] error[E0412]: cannot find type `CrateInherentImpls` in module `rustc::ty`
[00:09:03]     |
[00:09:03]     |
[00:09:03] 19  | / macro_rules! arena_types {
[00:09:03] 20  | |     ($macro:path, $args:tt, $tcx:lifetime) => (
[00:09:03] 21  | |         $macro!($args, [
[00:09:03] 22  | |             [] layouts: rustc::ty::layout::LayoutDetails,
[00:09:03] ...   |
[00:09:03] 78  | |             [few] crate_inherent_impls: rustc::ty::CrateInherentImpls,
[00:09:03] ...   |
[00:09:03] 114 | |     )
[00:09:03] 115 | | }
[00:09:03] 115 | | }
[00:09:03]     | |_- in this expansion of `arena_types!`
[00:09:03] ...
[00:09:03] 168 |   arena_types!(impl_arena_allocatable, [], 'tcx);
[00:09:03] 
[00:09:03] error[E0412]: cannot find type `SymbolName` in module `ty`
[00:09:03]   --> src/librustc/middle/exported_symbols.rs:37:17
[00:09:03]    |
---
[00:09:03] 
[00:09:03] error[E0412]: cannot find type `SymbolName` in module `ty`
[00:09:03]   --> src/librustc/middle/exported_symbols.rs:41:61
[00:09:03]    |
[00:09:03] 41 |     pub fn symbol_name(&self, tcx: TyCtxt<'tcx, '_>) -> ty::SymbolName {
[00:09:03] help: possible candidate is found in another module, you can import it into scope
[00:09:03]    |
[00:09:03] 1  | use backtrace::SymbolName;
[00:09:03]    |
[00:09:03]    |
[00:09:03] 
[00:09:03] error[E0425]: cannot find value `prev_` in this scope
[00:09:03]    --> src/librustc/middle/region.rs:818:18
[00:09:03]     |
[00:09:03] 818 |     visitor.cx = prev_}
[00:09:03] 
[00:09:03] error[E0425]: cannot find value `prev_` in this scope
[00:09:03]    --> src/librustc/middle/region.rs:839:18
[00:09:03]     |
[00:09:03]     |
[00:09:03] 839 |     visitor.cx = prev_}
[00:09:03] 
[00:09:03] error[E0425]: cannot find value `pa` in this scope
[00:09:03]    --> src/librustc/middle/region.rs:855:86
[00:09:03]     |
[00:09:03]     |
[00:09:03] 855 |     debug!("resolve_pat - post-increment {} pat = {:?}", visitor.expr_and_pat_count, pa}
[00:09:03]     |                                                                                      ^^ help: a local variable with a similar name exists: `pat`
[00:09:03] 
[00:09:03] error[E0425]: cannot find value `prev_pare` in this scope
[00:09:03]    --> src/librustc/middle/region.rs:873:25
[00:09:03]     |
[00:09:03] 873 |     visitor.cx.parent = prev_pare}
[00:09:03]     |                         ^^^^^^^^^ help: a local variable with a similar name exists: `prev_parent`
[00:09:03] error[E0425]: cannot find value `prev_` in this scope
[00:09:03]    --> src/librustc/middle/region.rs:975:18
[00:09:03]     |
[00:09:03]     |
[00:09:03] 975 |     visitor.cx = prev_}
[00:09:03] 
[00:09:04] error[E0412]: cannot find type `SymbolName` in module `ty`
[00:09:04] error[E0412]: cannot find type `SymbolName` in module `ty`
[00:09:04]   --> src/librustc/ty/query/values.rs:23:32
[00:09:04]    |
[00:09:04] 23 | impl<'tcx> Value<'tcx> for ty::SymbolName {
[00:09:04] help: possible candidate is found in another module, you can import it into scope
[00:09:04]    |
[00:09:04] 1  | use backtrace::SymbolName;
[00:09:04]    |
[00:09:04]    |
[00:09:04] 
[00:09:04] error[E0422]: cannot find struct, variant or union type `SymbolName` in module `ty`
[00:09:04]   --> src/librustc/ty/query/values.rs:25:13
[00:09:04]    |
[00:09:04] 25 |         ty::SymbolName { name: InternedString::intern("<error>") }
[00:09:04] help: possible candidate is found in another module, you can import it into scope
[00:09:04]    |
[00:09:04] 1  | use backtrace::SymbolName;
[00:09:04]    |
[00:09:04]    |
[00:09:04] 
[00:09:04] error[E0412]: cannot find type `SymbolName` in module `ty`
[00:09:04]     --> src/librustc/query/mod.rs:483:59
[00:09:04]      |
[00:09:04] 32   |  / rustc_queries! {
[00:09:04] 34   |  |         /// Records the type of every item.
[00:09:04] 34   |  |         /// Records the type of every item.
[00:09:04] 35   |  |         query type_of(key: DefId) -> Ty<'tcx> {
[00:09:04] ...     |
[00:09:04] 483  |  |         query symbol_name(key: ty::Instance<'tcx>) -> ty::SymbolName {
[00:09:04] ...     |
[00:09:04] 1092 |  |     }
[00:09:04] 1093 |  | }
[00:09:04] 1093 |  | }
[00:09:04]      |  |_- in this expansion of `rustc_query_append!`
[00:09:04]     ::: src/librustc/ty/query/mod.rs:101:1
[00:09:04]      |
[00:09:04]      |
[00:09:04] 101  | /  rustc_query_append! { [define_queries!][ <'tcx>
[00:09:04] 103  | |          /// Runs analysis passes on the crate.
[00:09:04] 103  | |          /// Runs analysis passes on the crate.
[00:09:04] 104  | |          [] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
[00:09:04] 106  | |  ]}
[00:09:04]      | |___- in this macro invocation
[00:09:04] help: possible candidate is found in another module, you can import it into scope
[00:09:04]      |
[00:09:04]      |
[00:09:04] 1    | use backtrace::SymbolName;
[00:09:04]      |
[00:09:04] 
[00:09:04] error[E0425]: cannot find function `is_impl_trait_defn` in module `super`
[00:09:04]    --> src/librustc/ty/wf.rs:366:31
[00:09:04]     |
[00:09:04] 366 |                     if super::is_impl_trait_defn(self.infcx.tcx, did).is_none() {
[00:09:04]     |                               ^^^^^^^^^^^^^^^^^^ not found in `super`
[00:09:04] error[E0412]: cannot find type `SymbolName` in module `self`
[00:09:04]     --> src/librustc/ty/mod.rs:3405:36
[00:09:04]      |
[00:09:04]      |
[00:09:04] 3405 | impl_stable_hash_for!(struct self::SymbolName {
[00:09:04] help: possible candidate is found in another module, you can import it into scope
[00:09:04]      |
[00:09:04] 5    | use backtrace::SymbolName;
[00:09:04]      |
[00:09:04]      |
[00:09:04] 
[00:09:04] error[E0422]: cannot find struct, variant or union type `SymbolName` in module `self`
[00:09:04]     --> src/librustc/ty/mod.rs:3405:36
[00:09:04]      |
[00:09:04] 3405 | impl_stable_hash_for!(struct self::SymbolName {
[00:09:04] help: possible candidate is found in another module, you can import it into scope
[00:09:04]      |
[00:09:04] 5    | use backtrace::SymbolName;
[00:09:04]      |
[00:09:04]      |
[00:09:04] 
[00:09:05] error: unused import: `DefId`
[00:09:05]   --> src/librustc/ty/codec.rs:10:26
[00:09:05]    |
[00:09:05] 10 | use crate::hir::def_id::{DefId, CrateNum};
[00:09:05]    |
[00:09:05]    = note: `-D unused-imports` implied by `-D warnings`
[00:09:05] 
[00:09:05] 
[00:09:05] error: unused imports: `CanonicalVarInfo`, `CanonicalVarInfos`
[00:09:05]   --> src/librustc/ty/codec.rs:11:31
[00:09:05]    |
[00:09:05] 11 | use crate::infer::canonical::{CanonicalVarInfo, CanonicalVarInfos};
[00:09:05] 
[00:09:05] 
[00:09:05] error: unused import: `crate::ty::subst::SubstsRef`
[00:09:05]   --> src/librustc/ty/codec.rs:17:5
[00:09:05]    |
[00:09:05] 17 | use crate::ty::subst::SubstsRef;
[00:09:05] 
[00:09:05] error: unused import: `crate::mir::interpret::Allocation`
[00:09:05]   --> src/librustc/ty/codec.rs:18:5
[00:09:05]    |
[00:09:05]    |
[00:09:05] 18 | use crate::mir::interpret::Allocation;
[00:09:05]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:09:05] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:504:16
[00:09:09]     |
[00:09:09] 504 | impl<'a, 'tcx> DecoderWithPosition for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:533:16
[00:09:09]     |
[00:09:09] 533 | impl<'a, 'tcx> ty_codec::TyDecoder<'tcx> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:593:16
[00:09:09]     |
[00:09:09] 593 | impl<'a, 'tcx> SpecializedDecoder<interpret::AllocId> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:600:16
[00:09:09]     |
[00:09:09] 600 | impl<'a, 'tcx> SpecializedDecoder<Span> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:663:16
[00:09:09]     |
[00:09:09] 663 | impl<'a, 'tcx> SpecializedDecoder<DefIndex> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:672:16
[00:09:09]     |
[00:09:09] 672 | impl<'a, 'tcx> SpecializedDecoder<DefId> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:683:16
[00:09:09]     |
[00:09:09] 683 | impl<'a, 'tcx> SpecializedDecoder<LocalDefId> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:690:16
[00:09:09]     |
[00:09:09] 690 | impl<'a, 'tcx> SpecializedDecoder<hir::HirId> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:717:16
[00:09:09]     |
[00:09:09] 717 | impl<'a, 'tcx> SpecializedDecoder<NodeId> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:725:16
[00:09:09]     |
[00:09:09] 725 | impl<'a, 'tcx> SpecializedDecoder<Fingerprint> for CacheDecoder<'a, 'tcx> {
[00:09:09]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:09] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:09]    --> src/librustc/ty/query/on_disk_cache.rs:731:30
[00:09:09]     |
[00:09:09] 731 | impl<'a, 'tcx, T: Decodable> SpecializedDecoder<mir::ClearCrossCrate<T>>
[00:09:09]     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:09] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:549:5
[00:09:10]     |
[00:09:10] 549 | /     fn cached_ty_for_shorthand<F>(&mut self,
[00:09:10] 550 | |                                   shorthand: usize,
[00:09:10] 551 | |                                   or_insert_with: F)
[00:09:10] 552 | |                                   -> Result<Ty<'tcx>, Self::Error>
[00:09:10] 569 | |         Ok(ty)
[00:09:10] 570 | |     }
[00:09:10] 570 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:594:5
[00:09:10]     |
[00:09:10] 594 | /     fn specialized_decode(&mut self) -> Result<interpret::AllocId, Self::Error> {
[00:09:10] 595 | |         let alloc_decoding_session = self.alloc_decoding_session;
[00:09:10] 596 | |         alloc_decoding_session.decode_alloc_id(self)
[00:09:10] 597 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:601:5
[00:09:10] 601 | /     fn specialized_decode(&mut self) -> Result<Span, Self::Error> {
[00:09:10] 601 | /     fn specialized_decode(&mut self) -> Result<Span, Self::Error> {
[00:09:10] 602 | |         let tag: u8 = Decodable::decode(self)?;
[00:09:10] 603 | |
[00:09:10] 604 | |         if tag == TAG_INVALID_SPAN {
[00:09:10] ...   |
[00:09:10] 655 | |         Ok(Span::new(lo, hi, ctxt))
[00:09:10] 656 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:664:5
[00:09:10]     |
[00:09:10] 664 | /     fn specialized_decode(&mut self) -> Result<DefIndex, Self::Error> {
[00:09:10] 665 | |         bug!("Trying to decode DefIndex outside the context of a DefId")
[00:09:10] 666 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:674:5
[00:09:10] 674 | /     fn specialized_decode(&mut self) -> Result<DefId, Self::Error> {
[00:09:10] 674 | /     fn specialized_decode(&mut self) -> Result<DefId, Self::Error> {
[00:09:10] 675 | |         // Load the DefPathHash which is was we encoded the DefId as.
[00:09:10] 676 | |         let def_path_hash = DefPathHash::decode(self)?;
[00:09:10] 677 | |
[00:09:10] 678 | |         // Using the DefPathHash, we can lookup the new DefId
[00:09:10] 679 | |         Ok(self.tcx().def_path_hash_to_def_id.as_ref().unwrap()[&def_path_hash])
[00:09:10] 680 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:685:5
[00:09:10]     |
[00:09:10] 685 | /     fn specialized_decode(&mut self) -> Result<LocalDefId, Self::Error> {
[00:09:10] 686 | |         Ok(LocalDefId::from_def_id(DefId::decode(self)?))
[00:09:10] 687 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:691:5
[00:09:10]     |
[00:09:10] 691 | /     fn specialized_decode(&mut self) -> Result<hir::HirId, Self::Error> {
[00:09:10] 692 | |         // Load the DefPathHash which is was we encoded the DefIndex as.
[00:09:10] 693 | |         let def_path_hash = DefPathHash::decode(self)?;
[00:09:10] ...   |
[00:09:10] 711 | |         })
[00:09:10] 712 | |     }
[00:09:10] 712 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:719:5
[00:09:10]     |
[00:09:10] 719 | /     fn specialized_decode(&mut self) -> Result<NodeId, Self::Error> {
[00:09:10] 720 | |         let hir_id = hir::HirId::decode(self)?;
[00:09:10] 721 | |         Ok(self.tcx().hir().hir_to_node_id(hir_id))
[00:09:10] 722 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:726:5
[00:09:10]     |
[00:09:10] 726 | /     fn specialized_decode(&mut self) -> Result<Fingerprint, Self::Error> {
[00:09:10] 727 | |         Fingerprint::decode_opaque(&mut self.opaque)
[00:09:10] 728 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:10] 
[00:09:10] error[E0277]: the trait bound `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>: rustc_serialize::Decoder` is not satisfied
[00:09:10]    --> src/librustc/ty/query/on_disk_cache.rs:735:5
[00:09:10]     |
[00:09:10] 735 | /     fn specialized_decode(&mut self) -> Result<mir::ClearCrossCrate<T>, Self::Error> {
[00:09:10] 736 | |         let discr = u8::decode(self)?;
[00:09:10] 738 | |         match discr {
[00:09:10] ...   |
[00:09:10] 747 | |         }
[00:09:10] 748 | |     }
[00:09:10] 748 | |     }
[00:09:10]     | |_____^ the trait `rustc_serialize::Decoder` is not implemented for `ty::query::on_disk_cache::CacheDecoder<'a, 'tcx>`
[00:09:11] error: aborting due to 53 previous errors
[00:09:11] 
[00:09:11] Some errors have detailed explanations: E0277, E0412, E0422, E0425, E0432.
[00:09:11] For more information about an error, try `rustc --explain E0277`.
---
travis_time:end:02343db0:start=1560288219650448471,finish=1560288219655488992,duration=5040521
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:011c324f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23e6ce2e
