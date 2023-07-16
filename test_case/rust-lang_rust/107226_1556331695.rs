
; cargo t -q
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:23:85
stack backtrace:
   0:        0x10089d864 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbcf32a37cac96a91
   1:        0x1008f0958 - core::fmt::write::he4ac7801a7c477c2
   2:        0x10089330c - std::io::Write::write_fmt::h22e5b10c231d60a2
   3:        0x10089d678 - std::sys_common::backtrace::print::h88d0a948d9efe2ff
   4:        0x1008a008c - std::panicking::default_hook::{{closure}}::h44b2e707537e2778
   5:        0x10089fe4c - std::panicking::default_hook::hfeb18c42faaa0956
   6:        0x108d82f78 - rustc_driver_impl[c07b475f7162e0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x1008a074c - std::panicking::rust_panic_with_hook::h934ded7e790c9958
   8:        0x1008a0504 - std::panicking::begin_panic_handler::{{closure}}::h7126a282c4e974b7
   9:        0x10089dc84 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7df1b8a9f804fcd
  10:        0x1008a02d8 - _rust_begin_unwind
  11:        0x10091c400 - core::panicking::panic_fmt::h0f564f53d93f5a9a
  12:        0x10091c470 - core::panicking::panic::h9a3a31a4d7c8524a
  13:        0x10c4e5200 - <rustc_metadata[6bb6e21c2fb6176b]::creader::CStore as rustc_session[820cacd699a5067b]::cstore::CrateStore>::def_path_hash_to_def_id
  14:        0x10ccac150 - <rustc_middle[723cfe9f1a3a538a]::ty::context::TyCtxt>::def_path_hash_to_def_id
  15:        0x10cbea9e4 - <rustc_query_system[d22d3f540ac00fd5]::dep_graph::dep_node::DepNode<rustc_middle[723cfe9f1a3a538a]::dep_graph::dep_node::DepKind> as rustc_middle[723cfe9f1a3a538a]::dep_graph::dep_node::DepNodeExt>::extract_def_id
  16:        0x10c100088 - <rustc_query_impl[7e39b3c0ea0e60de]::plumbing::query_callback<rustc_query_impl[7e39b3c0ea0e60de]::queries::opt_def_kind>::{closure#0} as core[7cd3bd47ed09ffd]::ops::function::FnOnce<(rustc_middle[723cfe9f1a3a538a]::ty::context::TyCtxt, rustc_query_system[d22d3f540ac00fd5]::dep_graph::dep_node::DepNode<rustc_middle[723cfe9f1a3a538a]::dep_graph::dep_node::DepKind>)>>::call_once
  17:        0x10c0ef534 - <rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepGraphData<rustc_middle[723cfe9f1a3a538a]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  18:        0x10c0ef57c - <rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepGraphData<rustc_middle[723cfe9f1a3a538a]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  19:        0x10c0ef57c - <rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepGraphData<rustc_middle[723cfe9f1a3a538a]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  20:        0x10c0ef57c - <rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepGraphData<rustc_middle[723cfe9f1a3a538a]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  21:        0x10c003370 - <std[ca6a5c052cb0c859]::thread::local::LocalKey<core[7cd3bd47ed09ffd]::cell::Cell<*const ()>>>::with::<rustc_middle[723cfe9f1a3a538a]::ty::context::tls::enter_context<rustc_query_system[d22d3f540ac00fd5]::query::plumbing::execute_job_incr<rustc_query_impl[7e39b3c0ea0e60de]::queries::evaluate_obligation, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>::{closure#1}, core[7cd3bd47ed09ffd]::option::Option<(rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 2usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core[7cd3bd47ed09ffd]::option::Option<(rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 2usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>>
  22:        0x10bf183e0 - rustc_query_system[d22d3f540ac00fd5]::query::plumbing::try_execute_query::<rustc_query_impl[7e39b3c0ea0e60de]::queries::evaluate_obligation, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  23:        0x10c169d6c - <rustc_query_impl[7e39b3c0ea0e60de]::Queries as rustc_middle[723cfe9f1a3a538a]::ty::query::QueryEngine>::evaluate_obligation
  24:        0x10c9eacc0 - <rustc_infer[cfb8c86e9d5860f5]::infer::InferCtxt as rustc_trait_selection[914d6aa5ea50c8f3]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  25:        0x10c9eb024 - <rustc_infer[cfb8c86e9d5860f5]::infer::InferCtxt as rustc_trait_selection[914d6aa5ea50c8f3]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  26:        0x10c927624 - <rustc_trait_selection[914d6aa5ea50c8f3]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  27:        0x10c926a64 - <rustc_trait_selection[914d6aa5ea50c8f3]::traits::fulfill::FulfillProcessor as rustc_data_structures[b6351f22e3d2cdd7]::obligation_forest::ObligationProcessor>::process_obligation
  28:        0x10c988fbc - <rustc_data_structures[b6351f22e3d2cdd7]::obligation_forest::ObligationForest<rustc_trait_selection[914d6aa5ea50c8f3]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[914d6aa5ea50c8f3]::traits::fulfill::FulfillProcessor>
  29:        0x10c9256b0 - <rustc_trait_selection[914d6aa5ea50c8f3]::traits::fulfill::FulfillmentContext as rustc_infer[cfb8c86e9d5860f5]::traits::engine::TraitEngine>::select_where_possible
  30:        0x10bc8bf98 - <dyn rustc_infer[cfb8c86e9d5860f5]::traits::engine::TraitEngine as rustc_infer[cfb8c86e9d5860f5]::traits::engine::TraitEngineExt>::select_all_or_error
  31:        0x10bcf4914 - rustc_traits[3f1e2625ab0cbba4]::codegen::codegen_select_candidate
  32:        0x10c0308fc - <std[ca6a5c052cb0c859]::thread::local::LocalKey<core[7cd3bd47ed09ffd]::cell::Cell<*const ()>>>::with::<rustc_middle[723cfe9f1a3a538a]::ty::context::tls::enter_context<rustc_query_system[d22d3f540ac00fd5]::query::plumbing::execute_job_incr<rustc_query_impl[7e39b3c0ea0e60de]::queries::codegen_select_candidate, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 16usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 16usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>
  33:        0x10bf3c964 - rustc_query_system[d22d3f540ac00fd5]::query::plumbing::try_execute_query::<rustc_query_impl[7e39b3c0ea0e60de]::queries::codegen_select_candidate, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  34:        0x10c15f2a8 - <rustc_query_impl[7e39b3c0ea0e60de]::Queries as rustc_middle[723cfe9f1a3a538a]::ty::query::QueryEngine>::codegen_select_candidate
  35:        0x10b18e35c - rustc_ty_utils[a741f169bbeee40b]::instance::inner_resolve_instance
  36:        0x10b18d898 - rustc_ty_utils[a741f169bbeee40b]::instance::resolve_instance
  37:        0x10bfec6b8 - <std[ca6a5c052cb0c859]::thread::local::LocalKey<core[7cd3bd47ed09ffd]::cell::Cell<*const ()>>>::with::<rustc_middle[723cfe9f1a3a538a]::ty::context::tls::enter_context<rustc_query_system[d22d3f540ac00fd5]::query::plumbing::execute_job_incr<rustc_query_impl[7e39b3c0ea0e60de]::queries::resolve_instance, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 32usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 32usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>
  38:        0x10bf05e70 - rustc_query_system[d22d3f540ac00fd5]::query::plumbing::try_execute_query::<rustc_query_impl[7e39b3c0ea0e60de]::queries::resolve_instance, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  39:        0x10c16bcc8 - <rustc_query_impl[7e39b3c0ea0e60de]::Queries as rustc_middle[723cfe9f1a3a538a]::ty::query::QueryEngine>::resolve_instance
  40:        0x10cc2c420 - <rustc_middle[723cfe9f1a3a538a]::ty::instance::Instance>::resolve_opt_const_arg
  41:        0x10cc2be40 - <rustc_middle[723cfe9f1a3a538a]::ty::instance::Instance>::expect_resolve
  42:        0x10b4a9da8 - <rustc_monomorphize[475d4823699b904d]::collector::MirNeighborCollector as rustc_middle[723cfe9f1a3a538a]::mir::visit::Visitor>::visit_terminator
  43:        0x10b4ae580 - rustc_monomorphize[475d4823699b904d]::collector::collect_neighbours
  44:        0x10b4acb88 - rustc_monomorphize[475d4823699b904d]::collector::collect_items_rec
  45:        0x10b4ad398 - rustc_monomorphize[475d4823699b904d]::collector::collect_items_rec
  46:        0x10b4ce454 - <core[7cd3bd47ed09ffd]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b6351f22e3d2cdd7]::sync::par_for_each_in<alloc[f5d5b4c8b72391a3]::vec::Vec<rustc_middle[723cfe9f1a3a538a]::mir::mono::MonoItem>, rustc_monomorphize[475d4823699b904d]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[7cd3bd47ed09ffd]::ops::function::FnOnce<()>>::call_once
  47:        0x10b4bf478 - rustc_data_structures[b6351f22e3d2cdd7]::sync::par_for_each_in::<alloc[f5d5b4c8b72391a3]::vec::Vec<rustc_middle[723cfe9f1a3a538a]::mir::mono::MonoItem>, rustc_monomorphize[475d4823699b904d]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  48:        0x10b4c6ff8 - <rustc_session[820cacd699a5067b]::session::Session>::time::<(), rustc_monomorphize[475d4823699b904d]::collector::collect_crate_mono_items::{closure#1}>
  49:        0x10b4ab170 - rustc_monomorphize[475d4823699b904d]::collector::collect_crate_mono_items
  50:        0x10b4bc624 - rustc_monomorphize[475d4823699b904d]::partitioning::collect_and_partition_mono_items
  51:        0x10c047144 - <std[ca6a5c052cb0c859]::thread::local::LocalKey<core[7cd3bd47ed09ffd]::cell::Cell<*const ()>>>::with::<rustc_middle[723cfe9f1a3a538a]::ty::context::tls::enter_context<rustc_query_system[d22d3f540ac00fd5]::query::plumbing::execute_job_incr<rustc_query_impl[7e39b3c0ea0e60de]::queries::collect_and_partition_mono_items, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 24usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[723cfe9f1a3a538a]::query::erase::Erased<[u8; 24usize]>, rustc_query_system[d22d3f540ac00fd5]::dep_graph::graph::DepNodeIndex)>
  52:        0x10bf4fb20 - rustc_query_system[d22d3f540ac00fd5]::query::plumbing::try_execute_query::<rustc_query_impl[7e39b3c0ea0e60de]::queries::collect_and_partition_mono_items, rustc_query_impl[7e39b3c0ea0e60de]::plumbing::QueryCtxt>
  53:        0x10c168ab4 - <rustc_query_impl[7e39b3c0ea0e60de]::Queries as rustc_middle[723cfe9f1a3a538a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  54:        0x108f18570 - rustc_codegen_ssa[28ee3fd94f9aaab4]::base::codegen_crate::<rustc_codegen_llvm[2145e776f1e6f672]::LlvmCodegenBackend>
  55:        0x108f15664 - <rustc_codegen_llvm[2145e776f1e6f672]::LlvmCodegenBackend as rustc_codegen_ssa[28ee3fd94f9aaab4]::traits::backend::CodegenBackend>::codegen_crate
  56:        0x108e26d20 - <rustc_session[820cacd699a5067b]::session::Session>::time::<alloc[f5d5b4c8b72391a3]::boxed::Box<dyn core[7cd3bd47ed09ffd]::any::Any>, rustc_interface[6cbd027d94b5de24]::passes::start_codegen::{closure#0}>
  57:        0x108e9d964 - rustc_interface[6cbd027d94b5de24]::passes::start_codegen
  58:        0x108e8b368 - <rustc_middle[723cfe9f1a3a538a]::ty::context::GlobalCtxt>::enter::<<rustc_interface[6cbd027d94b5de24]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[7cd3bd47ed09ffd]::result::Result<alloc[f5d5b4c8b72391a3]::boxed::Box<dyn core[7cd3bd47ed09ffd]::any::Any>, rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>>
  59:        0x108e55340 - <rustc_interface[6cbd027d94b5de24]::queries::Queries>::ongoing_codegen
  60:        0x108dd3bd0 - <rustc_interface[6cbd027d94b5de24]::interface::Compiler>::enter::<rustc_driver_impl[c07b475f7162e0]::run_compiler::{closure#1}::{closure#2}, core[7cd3bd47ed09ffd]::result::Result<core[7cd3bd47ed09ffd]::option::Option<rustc_interface[6cbd027d94b5de24]::queries::Linker>, rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>>
  61:        0x108d8a208 - rustc_span[61b8e85a80df2ab2]::set_source_map::<core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>, rustc_interface[6cbd027d94b5de24]::interface::run_compiler<core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>, rustc_driver_impl[c07b475f7162e0]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  62:        0x108d9df88 - std[ca6a5c052cb0c859]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6cbd027d94b5de24]::util::run_in_thread_pool_with_globals<rustc_interface[6cbd027d94b5de24]::interface::run_compiler<core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>, rustc_driver_impl[c07b475f7162e0]::run_compiler::{closure#1}>::{closure#0}, core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>>
  63:        0x108d8e9b4 - <<std[ca6a5c052cb0c859]::thread::Builder>::spawn_unchecked_<rustc_interface[6cbd027d94b5de24]::util::run_in_thread_pool_with_globals<rustc_interface[6cbd027d94b5de24]::interface::run_compiler<core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>, rustc_driver_impl[c07b475f7162e0]::run_compiler::{closure#1}>::{closure#0}, core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7cd3bd47ed09ffd]::result::Result<(), rustc_span[61b8e85a80df2ab2]::ErrorGuaranteed>>::{closure#1} as core[7cd3bd47ed09ffd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  64:        0x1008a8df4 - std::sys::unix::thread::Thread::new::thread_start::hb384a6c80d311b9f
  65:        0x1a1797fa8 - __pthread_joiner_wake

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-beta.4 (2013813b6 2023-05-07) running on aarch64-apple-darwin

note: compiler flags: --crate-type bin -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `core::iter::adapters::map::Map<conserve::merge::MergeTrees<conserve::index::IndexEntry, conserve::entry::EntryValue, readahead_iterator::Readahead<conserve::index::IndexEntry>, readahead_iterator::Readahead<conserve::entry::EntryValue>>, [closure@conserve::diff::diff::{closure#1}]>: core::iter::traits::iterator::Iterator`
#1 [codegen_select_candidate] computing candidate for `<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<conserve::merge::MergeTrees<conserve::index::IndexEntry, conserve::entry::EntryValue, readahead_iterator::Readahead<conserve::index::IndexEntry>, readahead_iterator::Readahead<conserve::entry::EntryValue>>, [closure@conserve::diff::diff::{closure#1}]>, [closure@conserve::diff::diff::{closure#2}]> as core::iter::traits::collect::IntoIterator>`
#2 [resolve_instance] resolving instance `<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<conserve::merge::MergeTrees<conserve::index::IndexEntry, conserve::entry::EntryValue, readahead_iterator::Readahead<conserve::index::IndexEntry>, readahead_iterator::Readahead<conserve::entry::EntryValue>>, [closure@conserve::diff::diff::{closure#1}]>, [closure@conserve::diff::diff::{closure#2}]> as core::iter::traits::collect::IntoIterator>::into_iter`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
there was a panic while trying to force a dep node
try_mark_green dep node stack:
#0 TraitSelect(2ce9722f4be39d84-e6abd6d79f6d982a)
#1 TraitSelect(f1545d48d51e5f25-cb50ec9c3fb605c0)
#2 TraitSelect(b770b3d75e5d9a83-fee98413a1acd933)
#3 evaluate_obligation(c4c407a854a36793-9a94dd7cff03eb78)
end of try_mark_green dep node stack
error: could not compile `conserve` (bin "conserve")
