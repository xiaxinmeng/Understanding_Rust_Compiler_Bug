
> 
RUST_BACKTRACE=1 rustc +stage1 -Zinstrument-coverage -Ztreat-err-as-bug src/test/ui/consts/unstable-precise-live-drops-in-libcore.rs
error[E0493]: destructors cannot be evaluated at compile-time
  --> src/test/ui/consts/unstable-precise-live-drops-in-libcore.rs:15:25
   |
15 |     pub const fn unwrap(self) -> T {
   |                         ^^^^ constant functions cannot evaluate destructors

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:990:27
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:519:12
   1: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
             at ./compiler/rustc_errors/src/lib.rs:990:27
   2: rustc_errors::HandlerInner::bump_err_count
             at ./compiler/rustc_errors/src/lib.rs:980:9
   3: rustc_errors::HandlerInner::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:796:13
   4: rustc_errors::Handler::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:715:9
   5: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
             at ./compiler/rustc_errors/src/diagnostic_builder.rs:101:9
   6: rustc_mir::transform::check_consts::post_drop_elaboration::CheckLiveDrops::check_live_drop
             at ./compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs:59:9
   7: <rustc_mir::transform::check_consts::post_drop_elaboration::CheckLiveDrops as rustc_middle::mir::visit::Visitor>::visit_terminator
             at ./compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs:93:21
   8: <rustc_mir::transform::check_consts::post_drop_elaboration::CheckLiveDrops as rustc_middle::mir::visit::Visitor>::visit_basic_block_data
             at ./compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs:72:9
   9: rustc_middle::mir::visit::Visitor::super_body
             at ./compiler/rustc_middle/src/mir/visit.rs:259:21
  10: rustc_middle::mir::visit::Visitor::visit_body
             at ./compiler/rustc_middle/src/mir/visit.rs:78:17
  11: rustc_mir::transform::check_consts::post_drop_elaboration::check_live_drops
             at ./compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs:40:5
  12: rustc_mir::transform::mir_drops_elaborated_and_const_checked
             at ./compiler/rustc_mir/src/transform/mod.rs:341:5
  13: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_drops_elaborated_and_const_checked>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:200:9
  16: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:608:17
  17: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  18: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  19: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  20: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  21: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  22: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  23: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  24: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  25: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
  26: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
  27: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
  28: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  29: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
  30: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  31: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  32: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  33: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  34: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  35: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  36: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  37: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  38: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  39: rustc_query_system::query::plumbing::ensure_query_impl
  40: rustc_query_system::query::plumbing::ensure_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:749:5
  41: rustc_middle::ty::query::TyCtxtEnsure::mir_drops_elaborated_and_const_checked
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:404:17
  42: rustc_interface::passes::analysis::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:865:17
  43: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
  44: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
  45: rustc_interface::passes::analysis
             at ./compiler/rustc_interface/src/passes.rs:860:5
  46: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  47: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  48: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:334:9
  49: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:600:17
  50: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  51: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  52: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  53: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  54: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  55: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  56: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  57: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  58: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
  59: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
  60: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
  61: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  62: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
  63: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  64: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  65: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  66: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  67: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  68: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  69: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  70: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  71: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  72: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  73: rustc_middle::ty::query::TyCtxtAt::analysis
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  74: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::analysis
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  75: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:440:59
  76: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:725:42
  77: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  78: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  79: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  80: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:725:9
  81: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:440:13
  82: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:413:19
  83: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:341:22
  84: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:196:13
  85: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:765:5
  86: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:190:5
  87: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:212:12
  88: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:152:13
  89: scoped_tls::ScopedKey<T>::set
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  90: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:94:5
  91: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:150:9
  92: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z instrument-coverage -Z treat-err-as-bug

query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `Either::<T, T>::unwrap`
#1 [analysis] running analysis passes on this crate
end of query stack
