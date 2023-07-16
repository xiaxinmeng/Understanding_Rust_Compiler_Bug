
   0: backtrace::backtrace::libunwind::trace
             at /home/omer/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/omer/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1025
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:193
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /home/omer/rust/rust/src/liballoc/boxed.rs:983
  11: rustc_driver::report_ice
             at src/librustc_driver/lib.rs:1174
  12: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:475
  13: rust_begin_unwind
             at src/libstd/panicking.rs:375
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:82
  15: core::panicking::panic
             at src/libcore/panicking.rs:49
  16: core::option::Option<T>::unwrap
             at /home/omer/rust/rust/src/libcore/macros/mod.rs:15
  17: rustc_mir::borrow_check::diagnostics::conflict_errors::<impl rustc_mir::borrow_check::MirBorrowckCtxt>::report_use_of_moved_or_uninitialized
             at src/librustc_mir/borrow_check/diagnostics/conflict_errors.rs:243
  18: rustc_mir::borrow_check::MirBorrowckCtxt::check_if_full_path_is_moved
             at src/librustc_mir/borrow_check/mod.rs:1597
  19: rustc_mir::borrow_check::MirBorrowckCtxt::check_if_path_or_subpath_is_moved
             at src/librustc_mir/borrow_check/mod.rs:1641
  20: rustc_mir::borrow_check::MirBorrowckCtxt::consume_operand
             at src/librustc_mir/borrow_check/mod.rs:0
  21: rustc_mir::borrow_check::MirBorrowckCtxt::consume_rvalue
             at src/librustc_mir/borrow_check/mod.rs:0
  22: <rustc_mir::borrow_check::MirBorrowckCtxt as rustc_mir::dataflow::DataflowResultsConsumer>::visit_statement_entry
             at src/librustc_mir/borrow_check/mod.rs:519
  23: rustc_mir::dataflow::DataflowResultsConsumer::process_basic_block
             at src/librustc_mir/dataflow/mod.rs:345
  24: rustc_mir::dataflow::DataflowResultsConsumer::analyze_results
             at src/librustc_mir/dataflow/mod.rs:333
  25: rustc_mir::borrow_check::do_mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:298
  26: rustc_mir::borrow_check::mir_borrowck::{{closure}}
             at src/librustc_mir/borrow_check/mod.rs:92
  27: rustc::infer::InferCtxtBuilder::enter::{{closure}}
             at /home/omer/rust/rust/src/librustc/infer/mod.rs:541
  28: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1613
  29: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1835
  30: rustc::ty::context::tls::set_tlv
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1768
  31: rustc::ty::context::tls::enter_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1834
  32: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1612
  33: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1941
  34: rustc::ty::context::tls::with_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
  35: rustc::ty::context::tls::with_context_opt
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1913
  36: rustc::ty::context::tls::with_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
  37: rustc::ty::context::tls::with_related_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1937
  38: rustc::ty::context::GlobalCtxt::enter_local
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1604
  39: rustc::infer::InferCtxtBuilder::enter
             at /home/omer/rust/rust/src/librustc/infer/mod.rs:540
  40: rustc_mir::borrow_check::mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:89
  41: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:984
  42: rustc::ty::query::__query_compute::mir_borrowck
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:935
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:976
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/omer/rust/rust/src/librustc/dep_graph/graph.rs:322
  45: rustc::dep_graph::graph::DepGraph::with_task
             at /home/omer/rust/rust/src/librustc/dep_graph/graph.rs:198
  46: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:548
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:280
  48: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1835
  49: rustc::ty::context::tls::set_tlv
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1768
  50: rustc::ty::context::tls::enter_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1834
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:279
  52: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1941
  53: rustc::ty::context::tls::with_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
  54: rustc::ty::context::tls::with_context_opt
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1913
  55: rustc::ty::context::tls::with_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
  56: rustc::ty::context::tls::with_related_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1937
  57: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:268
  58: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:540
  59: rustc::ty::query::plumbing::with_diagnostics
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:213
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:539
  61: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:376
  62: rustc::ty::query::TyCtxtAt::mir_borrowck
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:1061
  63: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::mir_borrowck
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:1053
  64: rustc_mir::borrow_check::nll::type_check::TypeChecker::prove_closure_bounds
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:2644
  65: rustc_mir::borrow_check::nll::type_check::TypeChecker::prove_aggregate_predicates
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:2625
  66: rustc_mir::borrow_check::nll::type_check::TypeChecker::check_aggregate_rvalue
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:2422
  67: rustc_mir::borrow_check::nll::type_check::TypeChecker::check_rvalue
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:2005
  68: rustc_mir::borrow_check::nll::type_check::TypeChecker::check_stmt
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:1453
  69: rustc_mir::borrow_check::nll::type_check::TypeChecker::typeck_mir
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:2786
  70: rustc_mir::borrow_check::nll::type_check::type_check_internal
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:225
  71: rustc_mir::borrow_check::nll::type_check::type_check
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:162
  72: rustc_mir::borrow_check::nll::compute_regions
             at src/librustc_mir/borrow_check/nll/mod.rs:187
  73: rustc_mir::borrow_check::do_mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:203
  74: rustc_mir::borrow_check::mir_borrowck::{{closure}}
             at src/librustc_mir/borrow_check/mod.rs:92
  75: rustc::infer::InferCtxtBuilder::enter::{{closure}}
             at /home/omer/rust/rust/src/librustc/infer/mod.rs:541
  76: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1613
  77: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1835
  78: rustc::ty::context::tls::set_tlv
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1768
  79: rustc::ty::context::tls::enter_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1834
  80: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1612
  81: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1941
  82: rustc::ty::context::tls::with_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
  83: rustc::ty::context::tls::with_context_opt
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1913
  84: rustc::ty::context::tls::with_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
  85: rustc::ty::context::tls::with_related_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1937
  86: rustc::ty::context::GlobalCtxt::enter_local
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1604
  87: rustc::infer::InferCtxtBuilder::enter
             at /home/omer/rust/rust/src/librustc/infer/mod.rs:540
  88: rustc_mir::borrow_check::mir_borrowck
             at src/librustc_mir/borrow_check/mod.rs:89
  89: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:984
  90: rustc::ty::query::__query_compute::mir_borrowck
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:935
  91: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:976
  92: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/omer/rust/rust/src/librustc/dep_graph/graph.rs:322
  93: rustc::dep_graph::graph::DepGraph::with_task
             at /home/omer/rust/rust/src/librustc/dep_graph/graph.rs:198
  94: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:548
  95: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:280
  96: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1835
  97: rustc::ty::context::tls::set_tlv
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1768
  98: rustc::ty::context::tls::enter_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1834
  99: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:279
 100: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1941
 101: rustc::ty::context::tls::with_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
 102: rustc::ty::context::tls::with_context_opt
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1913
 103: rustc::ty::context::tls::with_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
 104: rustc::ty::context::tls::with_related_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1937
 105: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:268
 106: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:540
 107: rustc::ty::query::plumbing::with_diagnostics
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:213
 108: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:539
 109: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:376
 110: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:597
 111: rustc::ty::query::TyCtxtEnsure::mir_borrowck
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:1012
 112: rustc_interface::passes::analysis::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:855
 113: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/mod.rs:2752
 114: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at /home/omer/rust/rust/src/libcore/iter/traits/iterator.rs:629
 115: core::iter::traits::iterator::Iterator::fold::ok::{{closure}}
             at /home/omer/rust/rust/src/libcore/iter/traits/iterator.rs:1829
 116: core::iter::traits::iterator::Iterator::try_fold
             at /home/omer/rust/rust/src/libcore/iter/traits/iterator.rs:1710
 117: core::iter::traits::iterator::Iterator::fold
             at /home/omer/rust/rust/src/libcore/iter/traits/iterator.rs:1832
 118: core::iter::traits::iterator::Iterator::for_each
             at /home/omer/rust/rust/src/libcore/iter/traits/iterator.rs:632
 119: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
             at /home/omer/rust/rust/src/librustc/ty/mod.rs:2751
 120: rustc_interface::passes::analysis::{{closure}}
             at src/librustc_interface/passes.rs:855
 121: rustc::util::common::time_ext
             at /home/omer/rust/rust/src/librustc/util/common.rs:51
 122: rustc::util::common::time
             at /home/omer/rust/rust/src/librustc/util/common.rs:45
 123: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:854
 124: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:984
 125: rustc::ty::query::__query_compute::analysis
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:935
 126: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/omer/rust/rust/src/librustc/dep_graph/graph.rs:322
 127: rustc::dep_graph::graph::DepGraph::with_eval_always_task
             at /home/omer/rust/rust/src/librustc/dep_graph/graph.rs:374
 128: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:542
 129: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:280
 130: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1835
 131: rustc::ty::context::tls::set_tlv
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1768
 132: rustc::ty::context::tls::enter_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1834
 133: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:279
 134: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1941
 135: rustc::ty::context::tls::with_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
 136: rustc::ty::context::tls::with_context_opt
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1913
 137: rustc::ty::context::tls::with_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1924
 138: rustc::ty::context::tls::with_related_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1937
 139: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:268
 140: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:540
 141: rustc::ty::query::plumbing::with_diagnostics
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:213
 142: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:539
 143: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:376
 144: rustc::ty::query::TyCtxtAt::analysis
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:1061
 145: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::analysis
             at /home/omer/rust/rust/src/librustc/ty/query/plumbing.rs:1053
 146: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:395
 147: rustc_interface::passes::QueryContext::enter::{{closure}}
             at /home/omer/rust/rust/src/librustc_interface/passes.rs:730
 148: rustc::ty::context::tls::enter_global::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1867
 149: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1835
 150: rustc::ty::context::tls::set_tlv
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1768
 151: rustc::ty::context::tls::enter_context
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1834
 152: rustc::ty::context::tls::enter_global
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1866
 153: rustc_interface::passes::QueryContext::enter
             at /home/omer/rust/rust/src/librustc_interface/passes.rs:730
 154: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:395
 155: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at /home/omer/rust/rust/src/librustc_interface/queries.rs:333
 156: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:297
 157: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at /home/omer/rust/rust/src/librustc_interface/interface.rs:179
 158: rustc_interface::interface::run_compiler::{{closure}}
             at /home/omer/rust/rust/src/librustc_interface/interface.rs:188
 159: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc_interface/util.rs:163
 160: rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1823
 161: std::thread::local::LocalKey<T>::try_with
             at /home/omer/rust/rust/src/libstd/thread/local.rs:262
 162: std::thread::local::LocalKey<T>::with
             at /home/omer/rust/rust/src/libstd/thread/local.rs:239
 163: rustc::ty::context::tls::with_thread_locals::{{closure}}
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1815
 164: std::thread::local::LocalKey<T>::try_with
             at /home/omer/rust/rust/src/libstd/thread/local.rs:262
 165: std::thread::local::LocalKey<T>::with
             at /home/omer/rust/rust/src/libstd/thread/local.rs:239
 166: rustc::ty::context::tls::with_thread_locals
             at /home/omer/rust/rust/src/librustc/ty/context.rs:1807
 167: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc_interface/util.rs:163
 168: scoped_tls::ScopedKey<T>::set
             at /home/omer/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 169: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at /home/omer/rust/rust/src/librustc_interface/util.rs:159
 170: scoped_tls::ScopedKey<T>::set
             at /home/omer/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 171: syntax::with_globals::{{closure}}
             at /home/omer/rust/rust/src/libsyntax/lib.rs:64
 172: scoped_tls::ScopedKey<T>::set
             at /home/omer/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 173: syntax::with_globals
             at /home/omer/rust/rust/src/libsyntax/lib.rs:63
 174: rustc_interface::util::spawn_thread_pool::{{closure}}
             at /home/omer/rust/rust/src/librustc_interface/util.rs:158
 175: rustc_interface::util::scoped_thread::{{closure}}
             at /home/omer/rust/rust/src/librustc_interface/util.rs:135
