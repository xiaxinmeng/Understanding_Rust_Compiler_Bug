log
thread 'rustc' panicked at 'index out of bounds: the len is 109 but the index is 109', compiler/rustc_query_impl/src/on_disk_cache.rs:712:18
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_bounds_check
   3: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   4: <rustc_middle::ty::VariantDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   5: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::ty::VariantDef>, <alloc::vec::Vec<rustc_middle::ty::VariantDef> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
   6: <rustc_middle::ty::adt::AdtDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   7: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   8: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   9: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_middle::ty::codec::TyDecoder>::cached_ty_for_shorthand::<<&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  10: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  11: <rustc_middle::ty::subst::GenericArg as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  12: <core::result::Result<rustc_middle::ty::subst::GenericArg, alloc::string::String> as rustc_middle::ty::context::InternIteratorElement<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::intern_with::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, <rustc_middle::ty::context::TyCtxt>::mk_substs<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>::{closure#0}>
  13: <rustc_middle::traits::ImplSource<()> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  14: <core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  15: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>
  16: <<rustc_query_impl::queries::codegen_fulfill_obligation as rustc_query_system::query::config::QueryDescription<rustc_query_impl::plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK::{closure#0} as core::ops::function::FnOnce<(rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex)>>::call_once
  17: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, (rustc_middle::ty::ParamEnv, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>), core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>
  18: rustc_data_structures::stack::ensure_sufficient_stack::<core::option::Option<(core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (rustc_middle::ty::ParamEnv, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>), core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>::{closure#2}>
  19: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::codegen_fulfill_obligation, rustc_query_impl::plumbing::QueryCtxt>
  20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fulfill_obligation
  21: rustc_ty_utils::instance::inner_resolve_instance
  22: rustc_ty_utils::instance::resolve_instance
  23: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::ParamEnvAnd<(rustc_span::def_id::DefId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>)>, core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>>
  24: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::ParamEnvAnd<(rustc_span::def_id::DefId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>)>, core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>>
  25: rustc_data_structures::stack::ensure_sufficient_stack::<core::option::Option<(core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::ty::ParamEnvAnd<(rustc_span::def_id::DefId, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>)>, core::result::Result<core::option::Option<rustc_middle::ty::instance::Instance>, rustc_errors::ErrorReported>>::{closure#2}>
  26: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::resolve_instance, rustc_query_impl::plumbing::QueryCtxt>
  27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance
  28: <rustc_middle::ty::instance::Instance>::resolve_opt_const_arg
  29: <rustc_middle::ty::instance::Instance>::resolve
  30: <rustc_monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator
  31: rustc_monomorphize::collector::collect_neighbours
  32: rustc_monomorphize::collector::collect_items_rec
  33: rustc_monomorphize::collector::collect_items_rec
  34: rustc_monomorphize::collector::collect_items_rec
  35: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  36: rustc_monomorphize::collector::collect_crate_mono_items
  37: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  38: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>
  39: rustc_data_structures::stack::ensure_sufficient_stack::<((&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit]), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>::{closure#3}>
  40: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  41: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  42: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  43: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  44: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  45: <rustc_interface::queries::Queries>::ongoing_codegen
  46: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  47: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  48: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (34926f0a1 2021-12-22) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `core::ops::try_trait::Try` fulfills its obligations
#1 [resolve_instance] resolving instance `<core::result::Result<command::Command, command::CommandError> as core::ops::try_trait::Try>::branch`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
warning: `repl` (bin "repl") generated 7 warnings
error: could not compile `repl`; 7 warnings emitted
