
thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1030:27
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:519:12
   1: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
             at ./compiler/rustc_errors/src/lib.rs:1030:27
   2: rustc_errors::HandlerInner::bump_err_count
             at ./compiler/rustc_errors/src/lib.rs:1020:9
   3: rustc_errors::HandlerInner::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:836:13
   4: rustc_errors::Handler::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:756:9
   5: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
             at ./compiler/rustc_errors/src/diagnostic_builder.rs:103:9
   6: rustc_resolve::macros::<impl rustc_resolve::Resolver>::compile_macro
             at ./compiler/rustc_resolve/src/macros.rs:1205:25
   7: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::get_macro_by_def_id
             at ./compiler/rustc_resolve/src/build_reduced_graph.rs:200:53
   8: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::get_macro
             at ./compiler/rustc_resolve/src/build_reduced_graph.rs:188:58
   9: rustc_resolve::macros::<impl rustc_resolve::Resolver>::check_reserved_macro_name
             at ./compiler/rustc_resolve/src/macros.rs:1176:30
  10: rustc_resolve::imports::<impl rustc_resolve::Resolver>::try_define
             at ./compiler/rustc_resolve/src/imports.rs:498:9
  11: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::define
             at ./compiler/rustc_resolve/src/build_reduced_graph.rs:92:35
  12: rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor::build_reduced_graph_for_external_crate_res
  13: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::build_reduced_graph_external
             at ./compiler/rustc_resolve/src/build_reduced_graph.rs:224:13
  14: rustc_resolve::Resolver::resolutions
             at ./compiler/rustc_resolve/src/lib.rs:1572:13
  15: rustc_resolve::Resolver::resolution
             at ./compiler/rustc_resolve/src/lib.rs:1582:10
  16: rustc_resolve::imports::<impl rustc_resolve::Resolver>::resolve_ident_in_module_unadjusted_ext
             at ./compiler/rustc_resolve/src/imports.rs:265:13
  17: rustc_resolve::Resolver::resolve_ident_in_module_ext
             at ./compiler/rustc_resolve/src/lib.rs:2012:9
  18: rustc_resolve::Resolver::resolve_ident_in_module
             at ./compiler/rustc_resolve/src/lib.rs:1982:9
  19: rustc_resolve::Resolver::resolve_path_with_ribs::{{closure}}
             at ./compiler/rustc_resolve/src/lib.rs:2239:21
  20: rustc_resolve::Resolver::resolve_path_with_ribs
             at ./compiler/rustc_resolve/src/lib.rs:2287:33
  21: rustc_resolve::Resolver::resolve_path
             at ./compiler/rustc_resolve/src/lib.rs:2112:9
  22: rustc_resolve::Resolver::resolve_ast_path
             at ./compiler/rustc_resolve/src/lib.rs:3224:15
  23: rustc_resolve::Resolver::resolve_str_path_error
             at ./compiler/rustc_resolve/src/lib.rs:3213:19
  24: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve_macro::{{closure}}
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:449:17
  25: rustc_interface::passes::BoxedResolver::access::{{closure}}
             at ./compiler/rustc_data_structures/src/box_region.rs:113:34
  26: rustc_interface::passes::configure_and_expand::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:138:9
  27: alloc::boxed::<impl core::ops::generator::Generator<R> for core::pin::Pin<alloc::boxed::Box<G,A>>>::resume
             at ./library/alloc/src/boxed.rs:1668:9
  28: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
             at ./compiler/rustc_data_structures/src/box_region.rs:55:13
  29: rustc_interface::passes::BoxedResolver::access
             at ./compiler/rustc_data_structures/src/box_region.rs:119:21
  30: rustdoc::core::DocContext::enter_resolver
             at ./src/librustdoc/core.rs:104:9
  31: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve_macro
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:431:9
  32: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve_with_disambiguator
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:1368:31
  33: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve_with_disambiguator_cached
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:1282:19
  34: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve_link
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:1091:39
  35: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:906:28
  36: rustdoc::fold::DocFolder::fold_mod::{{closure}}
             at ./src/librustdoc/fold.rs:85:55
  37: core::iter::adapters::filter_map::filter_map_try_fold::{{closure}}
             at ./library/core/src/iter/adapters/filter_map.rs:46:28
  38: core::iter::traits::iterator::Iterator::try_fold
             at ./library/core/src/iter/traits/iterator.rs:1993:21
  39: <core::iter::adapters::filter_map::FilterMap<I,F> as core::iter::traits::iterator::Iterator>::try_fold
             at ./library/core/src/iter/adapters/filter_map.rs:77:9
  40: <I as alloc::vec::source_iter_marker::SpecInPlaceCollect<T,I>>::collect_in_place
             at ./library/alloc/src/vec/source_iter_marker.rs:117:13
  41: alloc::vec::source_iter_marker::<impl alloc::vec::spec_from_iter::SpecFromIter<T,I> for alloc::vec::Vec<T>>::from_iter
             at ./library/alloc/src/vec/source_iter_marker.rs:55:19
  42: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at ./library/alloc/src/vec/mod.rs:2398:9
  43: core::iter::traits::iterator::Iterator::collect
             at ./library/core/src/iter/traits/iterator.rs:1775:9
  44: rustdoc::fold::DocFolder::fold_mod
             at ./src/librustdoc/fold.rs:85:20
  45: rustdoc::fold::DocFolder::fold_inner_recur
             at ./src/librustdoc/fold.rs:26:41
  46: rustdoc::fold::DocFolder::fold_item_recur
             at ./src/librustdoc/fold.rs:77:18
  47: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:918:23
  48: rustdoc::fold::DocFolder::fold_crate
             at ./src/librustdoc/fold.rs:90:20
  49: rustdoc::passes::collect_intra_doc_links::collect_intra_doc_links
             at ./src/librustdoc/passes/collect_intra_doc_links.rs:49:5
  50: rustdoc::core::run_global_ctxt::{{closure}}
             at ./src/librustdoc/core.rs:577:56
  51: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:573:9
  52: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:10:9
  53: rustdoc::core::run_global_ctxt
             at ./src/librustdoc/core.rs:577:21
  54: rustdoc::main_options::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustdoc/lib.rs:662:21
  55: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:573:9
  56: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:10:9
  57: rustdoc::main_options::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustdoc/lib.rs:661:55
  58: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:755:42
  59: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1726:50
  60: rustc_rayon_core::tlv::with
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/tlv.rs:19:5
  61: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1684:9
  62: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1726:9
  63: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:755:9
  64: rustdoc::main_options::{{closure}}::{{closure}}
             at ./src/librustdoc/lib.rs:660:13
  65: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:422:19
  66: rustdoc::main_options::{{closure}}
             at ./src/librustdoc/lib.rs:646:9
  67: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:208:13
  68: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:788:5
  69: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:202:5
  70: rustdoc::main_options
             at ./src/librustdoc/lib.rs:645:5
  71: rustdoc::main_args::{{closure}}
             at ./src/librustdoc/lib.rs:570:17
  72: rustc_rayon_core::thread_pool::ThreadPool::install::{{closure}}
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/thread_pool/mod.rs:110:40
  73: rustc_rayon_core::registry::Registry::in_worker_cold::{{closure}}::{{closure}}
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/registry.rs:468:21
  74: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute::call::{{closure}}
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/job.rs:116:21
  75: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./library/std/src/panic.rs:344:9
  76: std::panicking::try::do_call
             at ./library/std/src/panicking.rs:379:40
  77: std::panicking::try
             at ./library/std/src/panicking.rs:343:19
  78: std::panic::catch_unwind
             at ./library/std/src/panic.rs:431:14
  79: rustc_rayon_core::unwind::halt_unwinding
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/unwind.rs:17:5
  80: <rustc_rayon_core::job::StackJob<L,F,R> as rustc_rayon_core::job::Job>::execute
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/job.rs:123:38
  81: rustc_rayon_core::job::JobRef::execute
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/job.rs:60:9
  82: rustc_rayon_core::registry::WorkerThread::execute
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/registry.rs:726:9
  83: rustc_rayon_core::registry::WorkerThread::wait_until_cold
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/registry.rs:707:17
  84: rustc_rayon_core::registry::WorkerThread::wait_until
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/registry.rs:681:13
  85: rustc_rayon_core::registry::main_loop
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/registry.rs:805:5
  86: rustc_rayon_core::registry::ThreadBuilder::run
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/registry.rs:57:18
  87: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:214:21
  88: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  89: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:212:17
  90: rustc_rayon_core::ThreadPoolBuilder::build_scoped::{{closure}}::{{closure}}::{{closure}}
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.3.1/src/lib.rs:307:44
  91: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.7.2/src/thread.rs:415:31
  92: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.7.2/src/thread.rs:423:39
  93: <alloc::boxed::Box<F,A> as core::ops::function::FnMut<Args>>::call_mut
             at ./library/alloc/src/boxed.rs:1553:9
  94: crossbeam_utils::thread::ScopedThreadBuilder::spawn::{{closure}}
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.7.2/src/thread.rs:431:44
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
