
error: internal compiler error: librustc_metadata/decoder.rs:834: cannot get associated-item of `DefKey { parent: Some(DefIndex(0:7)), disambiguated_data: DisambiguatedDefPathData { data: AssocExistentialInImpl("Tmp"), disambiguator: 0 } }`

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:586:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:481
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_associated_item
  15: rustc_metadata::cstore_impl::provide_extern::associated_item
  16: rustc::ty::query::__query_compute::associated_item
  17: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::associated_item<'tcx>>::compute
  18: rustc::ty::context::tls::with_context
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  23: <core::iter::Map<I, F> as core::iter::iterator::Iterator>::next
  24: rustc::traits::project::assoc_ty_def
  25: rustc::infer::InferCtxt::commit_if_ok
  26: rustc::traits::project::opt_normalize_projection_type
  27: rustc::traits::project::normalize_projection_type
  28: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  29: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  30: rustc::ty::fold::TypeFoldable::fold_with
  31: rustc::infer::InferCtxt::partially_normalize_associated_types_in
  32: rustc_typeck::check::method::confirm::ConfirmContext::confirm
  33: rustc_typeck::check::FnCtxt::check_expr_kind
  34: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  35: rustc_typeck::check::FnCtxt::check_block_with_expected
  36: rustc_typeck::check::FnCtxt::check_expr_kind
  37: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  38: rustc_typeck::check::FnCtxt::check_return_expr
  39: rustc_typeck::check::check_fn
  40: rustc::ty::context::tls::with_related_context
  41: rustc::infer::InferCtxtBuilder::enter
  42: rustc_typeck::check::typeck_tables_of
  43: rustc::ty::query::__query_compute::typeck_tables_of
  44: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  45: rustc::ty::context::tls::with_context
  46: rustc::dep_graph::graph::DepGraph::with_task_impl
  47: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  49: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  51: rustc_typeck::check::typeck_item_bodies
  52: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  53: rustc::ty::context::tls::with_context
  54: rustc::dep_graph::graph::DepGraph::with_task_impl
  55: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  56: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  57: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  58: rustc_typeck::check_crate
  59: rustc::ty::context::tls::enter_context
  60: <std::thread::local::LocalKey<T>>::with
  61: rustc::ty::context::TyCtxt::create_and_enter
  62: rustc_driver::driver::compile_input
  63: rustc_driver::run_compiler_with_pool
  64: <scoped_tls::ScopedKey<T>>::set
  65: <scoped_tls::ScopedKey<T>>::set
  66: syntax::with_globals
  67: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  68: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  69: rustc_driver::run
  70: rustc_driver::main
  71: std::rt::lang_start::{{closure}}
  72: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  73: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  74: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  75: main
  76: __libc_start_main
  77: <unknown>
query stack during panic:
#0 [associated_item] processing `<a::X as a::View>::Tmp`
#1 [typeck_tables_of] processing `main`
#2 [typeck_item_bodies] type-checking all item bodies
end of query stack
