
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/joshua/src/rust/src/librustc_hir/definitions.rs:358:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  11: rust_begin_unwind
             at src/libstd/panicking.rs:385
  12: core::panicking::panic_fmt
             at src/libcore/panicking.rs:86
  13: core::panicking::panic
             at src/libcore/panicking.rs:51
  14: rustc_middle::hir::map::Map::def_kind
             at src/librustc_middle/hir/map/mod.rs:0
  15: rustc_middle::hir::map::provide::{{closure}}
             at src/librustc_middle/hir/map/mod.rs:1070
  16: core::ops::function::FnOnce::call_once
             at ./src/libcore/ops/function.rs:232
  17: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::def_kind>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:362
  18: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
  19: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:0
  20: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
  21: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
  22: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
  23: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
  24: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1702
  25: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1686
  26: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1702
  27: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
  28: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1790
  29: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1774
  30: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1763
  31: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1774
  32: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1787
  33: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
  34: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
  35: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
  36: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
  37: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
  38: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
  39: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:111
  40: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
  41: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
  42: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:731
  43: rustc_middle::ty::query::TyCtxtAt::def_kind
             at ./src/librustc_middle/ty/query/plumbing.rs:472
  44: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::def_kind
             at ./src/librustc_middle/ty/query/plumbing.rs:433
  45: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/passes/collect_intra_doc_links.rs:400
  46: rustdoc::fold::DocFolder::fold_mod::{{closure}}
             at src/librustdoc/fold.rs:88
  47: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
             at ./src/libcore/ops/function.rs:274
  48: core::iter::traits::iterator::Iterator::find_map::check::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:2258
  49: core::iter::traits::iterator::Iterator::try_fold
             at ./src/libcore/iter/traits/iterator.rs:1889
  50: core::iter::traits::iterator::Iterator::find_map
             at ./src/libcore/iter/traits/iterator.rs:2264
  51: <core::iter::adapters::FilterMap<I,F> as core::iter::traits::iterator::Iterator>::next
             at ./src/libcore/iter/adapters/mod.rs:1128
  52: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
             at ./src/liballoc/vec.rs:2081
  53: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at ./src/liballoc/vec.rs:1983
  54: core::iter::traits::iterator::Iterator::collect
             at ./src/libcore/iter/traits/iterator.rs:1671
  55: rustdoc::fold::DocFolder::fold_mod
             at src/librustdoc/fold.rs:88
  56: rustdoc::fold::DocFolder::fold_inner_recur
             at src/librustdoc/fold.rs:26
  57: rustdoc::fold::DocFolder::fold_item_recur
             at src/librustdoc/fold.rs:79
  58: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/passes/collect_intra_doc_links.rs:727
  59: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_crate::{{closure}}
             at src/librustdoc/passes/collect_intra_doc_links.rs:741
  60: core::option::Option<T>::and_then
             at ./src/libcore/option.rs:672
  61: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_crate
             at src/librustdoc/passes/collect_intra_doc_links.rs:741
  62: rustdoc::passes::collect_intra_doc_links::collect_intra_doc_links
             at src/librustdoc/passes/collect_intra_doc_links.rs:41
  63: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:553
  64: rustc_middle::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1725
  65: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1702
  66: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1686
  67: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1702
  68: rustc_middle::ty::context::tls::enter_global
             at ./src/librustc_middle/ty/context.rs:1725
  69: rustc_interface::passes::QueryContext::enter
             at ./src/librustc_interface/passes.rs:712
  70: rustdoc::core::run_core::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:411
  71: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:385
  72: rustdoc::core::run_core::{{closure}}
             at src/librustdoc/core.rs:373
  73: rustc_interface::interface::run_compiler_in_existing_thread_pool::{{closure}}
             at ./src/librustc_interface/interface.rs:195
  74: rustc_span::with_source_map
             at ./src/librustc_span/lib.rs:726
  75: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:189
  76: rustdoc::core::run_core
             at src/librustdoc/core.rs:372
  77: rustdoc::rust_input::{{closure}}
             at src/librustdoc/lib.rs:523
  78: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libstd/panic.rs:318
  79: std::panicking::try::do_call
             at ./src/libstd/panicking.rs:297
  80: std::panicking::try
             at ./src/libstd/panicking.rs:274
  81: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
  82: rustc_driver::catch_fatal_errors
             at ./src/librustc_driver/lib.rs:1124
  83: rustdoc::rust_input
             at src/librustdoc/lib.rs:520
  84: rustdoc::main_options
             at src/librustdoc/lib.rs:484
  85: rustdoc::main_args::{{closure}}
             at src/librustdoc/lib.rs:450
  86: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:149
  87: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
  88: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:145
  89: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
  90: rustc_ast::attr::with_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:44
  91: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
  92: rustc_ast::attr::with_globals
             at ./src/librustc_ast/attr/mod.rs:44
  93: rustc_interface::util::spawn_thread_pool::{{closure}}
             at ./src/librustc_interface/util.rs:144
  94: rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:119
