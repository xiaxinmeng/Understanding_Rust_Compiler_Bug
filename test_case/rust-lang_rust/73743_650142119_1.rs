
error: internal compiler error: TyKind::Error constructed but no error reported

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:942:13
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
             at src/libstd/panicking.rs:217
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:520
  11: std::panicking::begin_panic
             at ./src/libstd/panicking.rs:450
  12: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
             at src/librustc_errors/lib.rs:942
  13: rustc_errors::HandlerInner::bump_err_count
             at src/librustc_errors/lib.rs:928
  14: rustc_errors::HandlerInner::emit_diagnostic
             at src/librustc_errors/lib.rs:768
  15: rustc_errors::HandlerInner::emit_diag_at_span
             at ./src/librustc_errors/lib.rs:875
  16: rustc_errors::HandlerInner::span_bug
             at ./src/librustc_errors/lib.rs:870
  17: rustc_errors::HandlerInner::delay_span_bug
             at ./src/librustc_errors/lib.rs:885
  18: rustc_errors::Handler::delay_span_bug
             at ./src/librustc_errors/lib.rs:627
  19: rustc_session::session::Session::delay_span_bug
             at ./src/librustc_session/session.rs:436
  20: rustc_middle::ty::context::TyCtxt::ty_error_with_message
             at src/librustc_middle/ty/context.rs:1176
  21: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_path
             at src/librustc_typeck/check/expr.rs:0
  22: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
             at src/librustc_typeck/check/expr.rs:238
  23: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
             at src/librustc_typeck/check/expr.rs:180
  24: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr
             at src/librustc_typeck/check/expr.rs:132
  25: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
             at src/librustc_typeck/check/callee.rs:67
  26: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
             at src/librustc_typeck/check/expr.rs:268
  27: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
             at src/librustc_typeck/check/expr.rs:180
  28: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr
             at src/librustc_typeck/check/expr.rs:132
  29: rustc_typeck::check::FnCtxt::check_stmt
             at src/librustc_typeck/check/mod.rs:4641
  30: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
             at src/librustc_typeck/check/mod.rs:4741
  31: rustc_typeck::check::FnCtxt::with_breakable_ctxt
             at src/librustc_typeck/check/mod.rs:5814
  32: rustc_typeck::check::FnCtxt::check_block_with_expected
             at src/librustc_typeck/check/mod.rs:4739
  33: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
             at src/librustc_typeck/check/expr.rs:267
  34: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
             at src/librustc_typeck/check/expr.rs:180
  35: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_hint
             at src/librustc_typeck/check/expr.rs:111
  36: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
             at src/librustc_typeck/check/expr.rs:702
  37: rustc_typeck::check::check_fn
             at src/librustc_typeck/check/mod.rs:1377
  38: rustc_typeck::check::typeck_tables_of_with_fallback::{{closure}}
             at src/librustc_typeck/check/mod.rs:1027
  39: rustc_typeck::check::InheritedBuilder::enter::{{closure}}
             at src/librustc_typeck/check/mod.rs:660
  40: rustc_infer::infer::InferCtxtBuilder::enter::{{closure}}
             at ./src/librustc_infer/infer/mod.rs:622
  41: rustc_middle::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1573
  42: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1739
  43: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1723
  44: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1739
  45: rustc_middle::ty::context::GlobalCtxt::enter_local::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1573
  46: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1827
  47: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1811
  48: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1800
  49: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1811
  50: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1824
  51: rustc_middle::ty::context::GlobalCtxt::enter_local
             at ./src/librustc_middle/ty/context.rs:1565
  52: rustc_infer::infer::InferCtxtBuilder::enter
             at ./src/librustc_infer/infer/mod.rs:621
  53: rustc_typeck::check::InheritedBuilder::enter
             at src/librustc_typeck/check/mod.rs:660
  54: rustc_typeck::check::typeck_tables_of_with_fallback
             at src/librustc_typeck/check/mod.rs:997
  55: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
  56: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:0
  57: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:72
  58: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
  59: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
  60: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:72
  61: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1739
  62: rustc_middle::ty::context::tls::set_tlv
             at src/librustc_middle/ty/context.rs:1723
  63: rustc_middle::ty::context::tls::enter_context
             at src/librustc_middle/ty/context.rs:1739
  64: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc_middle/ty/query/plumbing.rs:71
  65: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1827
  66: rustc_middle::ty::context::tls::with_context::{{closure}}
             at src/librustc_middle/ty/context.rs:1811
  67: rustc_middle::ty::context::tls::with_context_opt
             at src/librustc_middle/ty/context.rs:1800
  68: rustc_middle::ty::context::tls::with_context
             at src/librustc_middle/ty/context.rs:1811
  69: rustc_middle::ty::context::tls::with_related_context
             at src/librustc_middle/ty/context.rs:1824
  70: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at src/librustc_middle/ty/query/plumbing.rs:60
  71: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
  72: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
  73: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
  74: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
  75: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
  76: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:111
  77: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
  78: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
  79: rustc_query_system::query::plumbing::get_query
             at ./src/librustc_query_system/query/plumbing.rs:731
  80: rustc_middle::ty::query::TyCtxtAt::typeck_tables_of
             at src/librustc_middle/ty/query/plumbing.rs:472
  81: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::typeck_tables_of
             at src/librustc_middle/ty/query/plumbing.rs:433
  82: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::body_tables
             at src/librustc_middle/ty/mod.rs:2569
  83: rustc_lint::context::LateContext::tables::{{closure}}
             at src/librustc_lint/context.rs:686
  84: core::option::Option<T>::unwrap_or_else
             at ./src/libcore/option.rs:430
  85: rustc_lint::context::LateContext::tables
             at src/librustc_lint/context.rs:685
  86: <rustc_lint::builtin::BoxPointers as rustc_lint::passes::LateLintPass>::check_expr
             at src/librustc_lint/builtin.rs:147
  87: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc_lint::passes::LateLintPass>::check_expr
             at src/librustc_lint/passes.rs:114
  88: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_expr::{{closure}}
             at src/librustc_lint/late.rs:172
  89: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
             at src/librustc_lint/late.rs:63
  90: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_expr
             at src/librustc_lint/late.rs:171
  91: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_body
             at src/librustc_lint/late.rs:138
  92: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_nested_body
             at src/librustc_lint/late.rs:120
  93: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_fn
             at src/librustc_lint/late.rs:202
  94: rustc_hir::intravisit::walk_item
             at ./src/librustc_hir/intravisit.rs:561
  95: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}}
             at src/librustc_lint/late.rs:148
  96: rustc_lint::late::LateContextAndPass<T>::with_param_env
             at src/librustc_lint/late.rs:75
  97: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
             at src/librustc_lint/late.rs:146
  98: rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
             at src/librustc_lint/late.rs:63
  99: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item
             at src/librustc_lint/late.rs:145
 100: rustc_hir::intravisit::Visitor::visit_nested_item
             at ./src/librustc_ast/visit.rs:237
 101: rustc_hir::intravisit::walk_mod
             at ./src/librustc_hir/intravisit.rs:480
 102: rustc_lint::late::LateContextAndPass<T>::process_mod
             at src/librustc_lint/late.rs:81
 103: rustc_lint::late::late_lint_mod_pass
             at src/librustc_lint/late.rs:393
 104: rustc_lint::late::late_lint_mod
             at src/librustc_lint/late.rs:411
 105: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::lint_mod>::compute
             at ./src/librustc_middle/ty/query/plumbing.rs:362
 106: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./src/librustc_query_system/dep_graph/graph.rs:303
 107: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc_query_system/dep_graph/graph.rs:0
 108: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 109: stacker::maybe_grow
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.9/src/lib.rs:52
 110: rustc_data_structures::stack::ensure_sufficient_stack
             at ./src/librustc_data_structures/stack.rs:16
 111: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:72
 112: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1739
 113: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1723
 114: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1739
 115: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc_middle/ty/query/plumbing.rs:71
 116: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1827
 117: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1811
 118: rustc_middle::ty::context::tls::with_context_opt
             at ./src/librustc_middle/ty/context.rs:1800
 119: rustc_middle::ty::context::tls::with_context
             at ./src/librustc_middle/ty/context.rs:1811
 120: rustc_middle::ty::context::tls::with_related_context
             at ./src/librustc_middle/ty/context.rs:1824
 121: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./src/librustc_middle/ty/query/plumbing.rs:60
 122: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:589
 123: rustc_query_system::query::plumbing::with_diagnostics
             at ./src/librustc_query_system/query/plumbing.rs:296
 124: rustc_query_system::query::plumbing::force_query_with_job
             at ./src/librustc_query_system/query/plumbing.rs:588
 125: rustc_query_system::query::plumbing::try_execute_query
             at ./src/librustc_query_system/query/plumbing.rs:415
 126: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./src/librustc_query_system/query/plumbing.rs:639
 127: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./src/librustc_query_system/query/caches.rs:111
 128: rustc_query_system::query::plumbing::try_get_cached
             at ./src/librustc_query_system/query/plumbing.rs:369
 129: rustc_query_system::query::plumbing::get_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:631
 130: rustc_query_system::query::plumbing::ensure_query_impl
             at ./src/librustc_query_system/query/plumbing.rs:679
 131: rustc_query_system::query::plumbing::ensure_query
             at ./src/librustc_query_system/query/plumbing.rs:741
 132: rustc_middle::ty::query::TyCtxtEnsure::lint_mod
             at ./src/librustc_middle/ty/query/plumbing.rs:389
 133: rustc_lint::late::check_crate::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_lint/late.rs:498
 134: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at ./src/libcore/iter/traits/iterator.rs:655
 135: core::iter::traits::iterator::Iterator::fold
             at ./src/libcore/iter/traits/iterator.rs:2022
 136: core::iter::traits::iterator::Iterator::for_each
             at ./src/libcore/iter/traits/iterator.rs:658
 137: rustc_lint::late::check_crate::{{closure}}::{{closure}}
             at ./src/librustc_lint/late.rs:497
 138: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:573
 139: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
 140: rustc_lint::late::check_crate::{{closure}}
             at ./src/librustc_lint/late.rs:495
 141: rustc_data_structures::sync::join
             at ./src/librustc_data_structures/sync.rs:159
 142: rustc_lint::late::check_crate
             at ./src/librustc_lint/late.rs:487
 143: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:413
 144: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./src/librustc_data_structures/profiling.rs:573
 145: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./src/librustc_session/utils.rs:9
 146: rustdoc::core::run_core::{{closure}}::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:412
 147: rustc_middle::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1762
 148: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc_middle/ty/context.rs:1739
 149: rustc_middle::ty::context::tls::set_tlv
             at ./src/librustc_middle/ty/context.rs:1723
 150: rustc_middle::ty::context::tls::enter_context
             at ./src/librustc_middle/ty/context.rs:1739
 151: rustc_middle::ty::context::tls::enter_global
             at ./src/librustc_middle/ty/context.rs:1762
 152: rustc_interface::passes::QueryContext::enter
             at ./src/librustc_interface/passes.rs:750
 153: rustdoc::core::run_core::{{closure}}::{{closure}}
             at src/librustdoc/core.rs:411
 154: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:386
 155: rustdoc::core::run_core::{{closure}}
             at src/librustdoc/core.rs:373
 156: rustc_interface::interface::run_compiler_in_existing_thread_pool::{{closure}}
             at ./src/librustc_interface/interface.rs:195
 157: rustc_span::with_source_map
             at ./src/librustc_span/lib.rs:726
 158: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:189
 159: rustdoc::core::run_core
             at src/librustdoc/core.rs:372
 160: rustdoc::rust_input::{{closure}}
             at src/librustdoc/lib.rs:520
 161: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libstd/panic.rs:318
 162: std::panicking::try::do_call
             at ./src/libstd/panicking.rs:342
 163: std::panicking::try
             at ./src/libstd/panicking.rs:319
 164: std::panic::catch_unwind
             at ./src/libstd/panic.rs:394
 165: rustc_driver::catch_fatal_errors
             at ./src/librustc_driver/lib.rs:1125
 166: rustdoc::rust_input
             at src/librustdoc/lib.rs:517
 167: rustdoc::main_options
             at src/librustdoc/lib.rs:481
 168: rustdoc::main_args::{{closure}}
             at src/librustdoc/lib.rs:447
 169: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:149
 170: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 171: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:145
 172: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 173: rustc_ast::attr::with_globals::{{closure}}
             at ./src/librustc_ast/attr/mod.rs:44
 174: scoped_tls::ScopedKey<T>::set
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 175: rustc_ast::attr::with_globals
             at ./src/librustc_ast/attr/mod.rs:44
 176: rustc_interface::util::spawn_thread_pool::{{closure}}
             at ./src/librustc_interface/util.rs:144
 177: rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:119
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
