
thread 'rustc' panicked at 'no entry found for key', compiler/rustc_metadata/src/rmeta/decoder.rs:1624:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8f3238f898163f09726c3d2b2cc9bafb09da26f3/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/8f3238f898163f09726c3d2b2cc9bafb09da26f3/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_display
             at /rustc/8f3238f898163f09726c3d2b2cc9bafb09da26f3/library/core/src/panicking.rs:63:5
   3: core::panicking::panic_str
             at /rustc/8f3238f898163f09726c3d2b2cc9bafb09da26f3/library/core/src/panicking.rs:55:5
   4: core::option::expect_failed
             at /rustc/8f3238f898163f09726c3d2b2cc9bafb09da26f3/library/core/src/option.rs:1821:5
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
  15: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  16: <rustc_middle::ty::context::TyCtxt>::mk_substs::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>
  17: <rustc_middle::traits::ImplSource<()> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  18: <core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  19: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>
  20: <<rustc_query_impl::queries::codegen_fulfill_obligation as rustc_query_system::query::config::QueryDescription<rustc_query_impl::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core::ops::function::FnOnce<(rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  21: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, (rustc_middle::ty::ParamEnv, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>), core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>
  22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(rustc_middle::ty::ParamEnv, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>), core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>>
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fulfill_obligation
  24: rustc_ty_utils::instance::inner_resolve_instance
  25: rustc_ty_utils::instance::resolve_instance
  26: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::ParamEnvAnd<(rustc_span::def_id::DefId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>)>, core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>>
  27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::resolve_instance, rustc_query_impl::plumbing::QueryCtxt>
  28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance
  29: <rustc_middle::ty::instance::Instance>::resolve_opt_const_arg
  30: <rustc_middle::ty::instance::Instance>::resolve_for_vtable
  31: <core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_span::def_id::DefId>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_span::def_id::DefId, rustc_middle::ty::vtable::VtblEntry, (), rustc_trait_selection::traits::vtable_entries::{closure#0}::{closure#1}, core::iter::traits::iterator::Iterator::for_each::call<rustc_middle::ty::vtable::VtblEntry, <alloc::vec::Vec<rustc_middle::ty::vtable::VtblEntry> as alloc::vec::spec_extend::SpecExtend<rustc_middle::ty::vtable::VtblEntry, core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_span::def_id::DefId>>, rustc_trait_selection::traits::vtable_entries::{closure#0}::{closure#1}>>>::spec_extend::{closure#0}>::{closure#0}>::{closure#0}>
  32: rustc_trait_selection::traits::vtable_entries::{closure#0}
  33: rustc_trait_selection::traits::vtable_entries
  34: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>, &[rustc_middle::ty::vtable::VtblEntry]>::{closure#0}, &[rustc_middle::ty::vtable::VtblEntry]>
  35: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>, &[rustc_middle::ty::vtable::VtblEntry]>
  36: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::vtable_entries, rustc_query_impl::plumbing::QueryCtxt>
  37: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::vtable_entries
  38: rustc_monomorphize::collector::collect_neighbours
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
  62: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  63: rustc_monomorphize::collector::collect_crate_mono_items
  64: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  65: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>::{closure#0}, (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>
  66: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>
  67: rustc_data_structures::stack::ensure_sufficient_stack::<((&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit]), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>::{closure#3}>
  68: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  69: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  70: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  71: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  72: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  73: <rustc_interface::queries::Queries>::ongoing_codegen
  74: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  75: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  76: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (8f3238f89 2022-01-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `serde::de::Expected` fulfills its obligations
#1 [resolve_instance] resolving instance `<tanker_enclave_protocol::message::_::<impl serde::de::Deserialize<'de> for tanker_enclave_protocol::message::ExtraData>::deserialize::__Visitor as serde::de::Expected>::fmt`
#2 [vtable_entries] finding all vtable entries for trait serde::de::Expected
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
