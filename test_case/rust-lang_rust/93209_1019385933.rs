
   Compiling capnp-rpc-test v0.0.0 (/home/dwrensha/src/capnproto-rust/capnp-rpc/test)
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:755:13: Broken MIR: generator contains type RemotePromise<get_cap_results::Owned> in MIR, but typeck only knows about {ResumeTy, &test_capnp::bootstrap::Client, test_capnp::bootstrap::Client, RemotePromise<test_pipeline_results::Owned>, Promise<Response<test_pipeline_results::Owned>, capnp::Error>, (), Response<test_pipeline_results::Owned>, test_pipeline::Client, Request<get_cap_params::Owned, get_cap_results::Owned>, TestInterface, Rc<Cell<u64>>, Request<test_interface::foo_params::Owned, foo_results::Owned>, RemotePromise<foo_results::Owned>, RemotePromise<test_all_types::Owned>, Promise<Response<foo_results::Owned>, capnp::Error>, Response<foo_results::Owned>, Promise<Response<test_all_types::Owned>, capnp::Error>} and [test_capnp::bootstrap::Client]
   --> capnp-rpc/test/test.rs:247:49
    |
247 |       rpc_top_level(|_spawner, client| async move {
    |  _________________________________________________^
248 | |         let response = client.test_pipeline_request().send().promise.await?;
249 | |         let client = response.get()?.get_cap()?;
250 | |
...   |
285 | |         Ok(())
286 | |     });
    | |_____^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/17d29dcdce9b9e838635eb0adefd9b8b1588410b/compiler/rustc_errors/src/lib.rs:1115:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::span_bug::<rustc_span::span_encoding::Span>
   3: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span>
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::span_bug_fmt::<rustc_span::span_encoding::Span>
   7: <rustc_mir_transform::generator::StateTransform as rustc_middle::mir::MirPass>::run_pass
   8: rustc_mir_transform::pass_manager::run_passes
   9: rustc_mir_transform::optimized_mir
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, &rustc_middle::mir::Body>
  11: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::mir::Body, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::mir::Body>::{closure#3}>
  12: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  13: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  14: <rustc_middle::ty::context::TyCtxt>::generator_layout
  15: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached
  16: rustc_middle::ty::layout::layout_of
  17: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<&rustc_middle::ty::TyS>, core::result::Result<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError>>
  18: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  20: core::iter::adapters::process_results::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::FieldDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError, <core::result::Result<alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>, rustc_middle::ty::layout::LayoutError> as core::iter::traits::collect::FromIterator<core::result::Result<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError>>>::from_iter<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::FieldDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>>::{closure#0}, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>>
  21: core::iter::adapters::process_results::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::VariantDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>, rustc_middle::ty::layout::LayoutError, <core::result::Result<rustc_index::vec::IndexVec<rustc_target::abi::VariantIdx, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>>, rustc_middle::ty::layout::LayoutError> as core::iter::traits::collect::FromIterator<core::result::Result<alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>, rustc_middle::ty::layout::LayoutError>>>::from_iter<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::VariantDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>>::{closure#0}, rustc_index::vec::IndexVec<rustc_target::abi::VariantIdx, alloc::vec::Vec<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>>>>
  22: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached
  23: rustc_middle::ty::layout::layout_of
  24: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<&rustc_middle::ty::TyS>, core::result::Result<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError>>
  25: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  27: rustc_middle::ty::layout::layout_of
  28: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<&rustc_middle::ty::TyS>, core::result::Result<rustc_target::abi::TyAndLayout<&rustc_middle::ty::TyS>, rustc_middle::ty::layout::LayoutError>>
  29: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  30: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  31: <rustc_mir_transform::const_prop::ConstProp as rustc_middle::mir::MirPass>::run_pass
  32: rustc_mir_transform::pass_manager::run_passes
  33: rustc_mir_transform::optimized_mir
  34: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, &rustc_middle::mir::Body>
  35: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::mir::Body, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::mir::Body>::{closure#3}>
  36: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  37: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  38: <rustc_middle::ty::context::TyCtxt>::instance_mir
  39: rustc_monomorphize::collector::collect_neighbours
  40: rustc_monomorphize::collector::collect_items_rec
  41: rustc_monomorphize::collector::collect_items_rec
  42: rustc_monomorphize::collector::collect_items_rec
  43: rustc_monomorphize::collector::collect_items_rec
  44: rustc_monomorphize::collector::collect_items_rec
  45: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  46: rustc_monomorphize::collector::collect_crate_mono_items
  47: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  48: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>::{closure#0}, (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>
  49: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>
  50: rustc_data_structures::stack::ensure_sufficient_stack::<((&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit]), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>::{closure#3}>
  51: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  52: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  53: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  54: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  55: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  56: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
  57: <rustc_interface::queries::Queries>::ongoing_codegen
  58: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  59: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  60: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (17d29dcdc 2022-01-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `basic_pipelining::{closure#0}::{closure#0}`
#1 [layout_of] computing layout of `[static generator@capnp-rpc/test/test.rs:247:49: 286:6]`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@capnp-rpc/test/test.rs:247:49: 286:6]>`
#3 [layout_of] computing layout of `impl core::future::future::Future<Output = [async output]>`
#4 [optimized_mir] optimizing MIR for `basic_pipelining::{closure#0}`
#5 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `capnp-rpc-test`
