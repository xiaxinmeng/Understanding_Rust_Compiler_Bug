
thread 'rustc' panicked at 'no entry found for key', compiler/rustc_metadata/src/rmeta/decoder.rs:1624:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_display
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/core/src/panicking.rs:63:5
   3: core::panicking::panic_str
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/core/src/panicking.rs:55:5
   4: core::option::expect_failed
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/core/src/option.rs:1817:5
   5: <rustc_metadata::creader::CStore as rustc_session::cstore::CrateStore>::expn_hash_to_expn_id
   6: <rustc_span::hygiene::ExpnId as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   7: <rustc_span::hygiene::SyntaxContextData as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   8: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_middle::ty::codec::TyDecoder>::with_position::<<rustc_span::hygiene::SyntaxContext as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}::{closure#0}, core::result::Result<rustc_span::hygiene::SyntaxContextData, alloc::string::String>>
   9: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  10: <rustc_middle::ty::VariantDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  11: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::ty::VariantDef>, <alloc::vec::Vec<rustc_middle::ty::VariantDef> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  12: <rustc_middle::ty::adt::AdtDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  13: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  14: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  15: <rustc_middle::ty::context::TyCtxt>::mk_substs::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>
  16: <rustc_middle::traits::ImplSource<()> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  17: <core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  18: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>
  19: <<rustc_query_impl::queries::codegen_fulfill_obligation as rustc_query_system::query::config::QueryDescription<rustc_query_impl::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core::ops::function::FnOnce<(rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  20: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, (rustc_middle::ty::ParamEnv, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>), core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>
  21: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(rustc_middle::ty::ParamEnv, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>), core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>>
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fulfill_obligation
  23: rustc_ty_utils::instance::inner_resolve_instance
  24: rustc_ty_utils::instance::resolve_instance
  25: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::ParamEnvAnd<(rustc_span::def_id::DefId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>)>, core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>>
  26: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::resolve_instance, rustc_query_impl::plumbing::QueryCtxt>
  27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance
  28: <rustc_middle::ty::instance::Instance>::resolve_opt_const_arg
  29: <rustc_middle::ty::instance::Instance>::resolve_for_vtable
  30: <core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_span::def_id::DefId>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_span::def_id::DefId, rustc_middle::ty::vtable::VtblEntry, (), rustc_trait_selection::traits::vtable_entries::{closure#0}::{closure#1}, core::iter::traits::iterator::Iterator::for_each::call<rustc_middle::ty::vtable::VtblEntry, <alloc::vec::Vec<rustc_middle::ty::vtable::VtblEntry> as alloc::vec::spec_extend::SpecExtend<rustc_middle::ty::vtable::VtblEntry, core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_span::def_id::DefId>>, rustc_trait_selection::traits::vtable_entries::{closure#0}::{closure#1}>>>::spec_extend::{closure#0}>::{closure#0}>::{closure#0}>
  31: rustc_trait_selection::traits::vtable_entries::{closure#0}
  32: rustc_trait_selection::traits::vtable_entries
  33: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>, &[rustc_middle::ty::vtable::VtblEntry]>::{closure#0}, &[rustc_middle::ty::vtable::VtblEntry]>
  34: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>, &[rustc_middle::ty::vtable::VtblEntry]>
  35: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::vtable_entries, rustc_query_impl::plumbing::QueryCtxt>
  36: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::vtable_entries
  37: rustc_monomorphize::collector::collect_neighbours
  38: rustc_monomorphize::collector::collect_items_rec
  39: rustc_monomorphize::collector::collect_items_rec
  40: rustc_monomorphize::collector::collect_items_rec
  41: rustc_monomorphize::collector::collect_items_rec
  42: rustc_monomorphize::collector::collect_items_rec
  43: rustc_monomorphize::collector::collect_items_rec
  44: rustc_monomorphize::collector::collect_items_rec
  45: rustc_monomorphize::collector::collect_items_rec
  46: rustc_monomorphize::collector::collect_items_rec
  47: rustc_monomorphize::collector::collect_items_rec
  48: rustc_monomorphize::collector::collect_items_rec
  49: rustc_monomorphize::collector::collect_items_rec
  50: rustc_monomorphize::collector::collect_items_rec
  51: rustc_monomorphize::collector::collect_items_rec
  52: rustc_monomorphize::collector::collect_items_rec
  53: rustc_monomorphize::collector::collect_items_rec
  54: rustc_monomorphize::collector::collect_items_rec
  55: rustc_monomorphize::collector::collect_items_rec
  56: rustc_monomorphize::collector::collect_items_rec
  57: rustc_monomorphize::collector::collect_items_rec
  58: rustc_monomorphize::collector::collect_items_rec
  59: rustc_monomorphize::collector::collect_items_rec
  60: rustc_monomorphize::collector::collect_items_rec
  61: rustc_monomorphize::collector::collect_items_rec
  62: rustc_monomorphize::collector::collect_items_rec
  63: rustc_monomorphize::collector::collect_items_rec
  64: rustc_monomorphize::collector::collect_items_rec
  65: rustc_monomorphize::collector::collect_items_rec
  66: rustc_monomorphize::collector::collect_items_rec
  67: rustc_monomorphize::collector::collect_items_rec
  68: rustc_monomorphize::collector::collect_items_rec
  69: rustc_monomorphize::collector::collect_items_rec
  70: rustc_monomorphize::collector::collect_items_rec
  71: rustc_monomorphize::collector::collect_items_rec
  72: rustc_monomorphize::collector::collect_items_rec
  73: rustc_monomorphize::collector::collect_items_rec
  74: rustc_monomorphize::collector::collect_items_rec
  75: rustc_monomorphize::collector::collect_items_rec
  76: rustc_monomorphize::collector::collect_items_rec
  77: rustc_monomorphize::collector::collect_items_rec
  78: rustc_monomorphize::collector::collect_items_rec
  79: rustc_monomorphize::collector::collect_items_rec
  80: rustc_monomorphize::collector::collect_items_rec
  81: rustc_monomorphize::collector::collect_items_rec
  82: rustc_monomorphize::collector::collect_items_rec
  83: rustc_monomorphize::collector::collect_items_rec
  84: rustc_monomorphize::collector::collect_items_rec
  85: rustc_monomorphize::collector::collect_items_rec
  86: rustc_monomorphize::collector::collect_items_rec
  87: rustc_monomorphize::collector::collect_items_rec
  88: rustc_monomorphize::collector::collect_items_rec
  89: rustc_monomorphize::collector::collect_items_rec
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (399ba6bb3 2022-01-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `serde::de::Expected` fulfills its obligations
#1 [resolve_instance] resolving instance `<singularity::adlist::_::<impl serde::de::Deserialize<'de> for singularity::adlist::AdlistFormat>::deserialize::__FieldVisitor as serde::de::Expected>::fmt`
#2 [vtable_entries] finding all vtable entries for trait serde::de::Expected
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
