

error: expected identifier, found reserved identifier `_`
 --> src/lib.rs:2:26
  |
2 |     [0; match [|f @ &ref _| () ] {} ]
  |                          ^ expected identifier, found reserved identifier

error[E0658]: `match` is not allowed in a `const`
 --> src/lib.rs:2:9
  |
2 |     [0; match [|f @ &ref _| () ] {} ]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/49146
  = help: add `#![feature(const_if_match)]` to the crate attributes to enable

error: internal compiler error: src/librustc/ty/sty.rs:1815: `sequence_element_type` called on non-sequence value: [type error]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1024
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:190
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:207
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:470
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: rustc::ty::sty::<impl rustc::ty::TyS>::sequence_element_type
  21: rustc_mir::build::expr::as_rvalue::<impl rustc_mir::build::Builder>::expr_as_rvalue
  22: rustc_mir::build::expr::into::<impl rustc_mir::build::Builder>::into_expr
  23: rustc_mir::build::expr::as_temp::<impl rustc_mir::build::Builder>::expr_as_temp
  24: rustc_mir::build::expr::as_place::<impl rustc_mir::build::Builder>::expr_as_place
  25: rustc_mir::build::expr::as_place::<impl rustc_mir::build::Builder>::expr_as_place
  26: rustc_mir::build::expr::as_place::<impl rustc_mir::build::Builder>::as_place
  27: rustc_mir::build::expr::into::<impl rustc_mir::build::Builder>::into_expr
  28: rustc_mir::build::expr::as_temp::<impl rustc_mir::build::Builder>::expr_as_temp
  29: rustc_mir::build::expr::as_operand::<impl rustc_mir::build::Builder>::expr_as_operand
  30: rustc_mir::build::expr::as_rvalue::<impl rustc_mir::build::Builder>::expr_as_rvalue
  31: rustc_mir::build::expr::into::<impl rustc_mir::build::Builder>::into_expr
  32: rustc_mir::build::expr::into::<impl rustc_mir::build::Builder>::into_expr
  33: rustc_mir::build::expr::into::<impl rustc_mir::build::Builder>::into_expr
  34: rustc_mir::build::construct_const
  35: rustc::ty::context::tls::with_context::{{closure}}
  36: rustc_mir::build::mir_build
  37: rustc_mir::transform::mir_built
  38: rustc::ty::query::__query_compute::mir_built
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  40: rustc_mir::transform::check_unsafety::unsafety_check_result
  41: rustc::ty::query::__query_compute::unsafety_check_result
  42: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  43: rustc_mir::transform::mir_const
  44: rustc::ty::query::__query_compute::mir_const
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  46: rustc_mir::transform::mir_const_qualif
  47: rustc::ty::query::__query_compute::mir_const_qualif
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  49: rustc_mir::transform::mir_validated
  50: rustc::ty::query::__query_compute::mir_validated
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  52: rustc_mir::borrow_check::mir_borrowck
  53: rustc::ty::query::__query_compute::mir_borrowck
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  55: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  56: rustc_mir::transform::optimized_mir
  57: rustc::ty::query::__query_compute::optimized_mir
  58: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  59: rustc_mir::interpret::eval_context::InterpCx<M>::load_mir
  60: rustc_mir::const_eval::const_eval_raw_provider
  61: rustc::ty::query::__query_compute::const_eval_raw
  62: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  63: rustc_mir::const_eval::const_eval_provider
  64: rustc::ty::query::__query_compute::const_eval
  65: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  66: rustc::dep_graph::graph::DepGraph::with_task_impl
  67: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  68: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  69: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  70: rustc_typeck::check::FnCtxt::check_block_with_expected
  71: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  72: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  73: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  74: rustc_typeck::check::check_fn
  75: rustc::ty::context::tls::with_context::{{closure}}
  76: rustc_typeck::check::typeck_tables_of
  77: rustc::ty::query::__query_compute::typeck_tables_of
  78: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  79: rustc::dep_graph::graph::DepGraph::with_task_impl
  80: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  81: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  82: rustc_typeck::check::typeck_item_bodies
  83: rustc::ty::query::__query_compute::typeck_item_bodies
  84: rustc::dep_graph::graph::DepGraph::with_task_impl
  85: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  86: rustc_typeck::check_crate
  87: rustc_interface::passes::analysis
  88: rustc::ty::query::__query_compute::analysis
  89: rustc::dep_graph::graph::DepGraph::with_task_impl
  90: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  91: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  92: rustc_interface::passes::create_global_ctxt::{{closure}}
  93: rustc_interface::passes::BoxedGlobalCtxt::enter
  94: rustc_interface::interface::run_compiler_in_existing_thread_pool
  95: std::thread::local::LocalKey<T>::with
  96: scoped_tls::ScopedKey<T>::set
  97: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (bbb664a99 2019-11-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_built] processing `bug::{{constant}}#0`
#1 [unsafety_check_result] processing `bug::{{constant}}#0`
#2 [mir_const] processing `bug::{{constant}}#0`
#3 [mir_const_qualif] const checking `bug::{{constant}}#0`
#4 [mir_validated] processing `bug::{{constant}}#0`
#5 [mir_borrowck] processing `bug::{{constant}}#0`
#6 [optimized_mir] processing `bug::{{constant}}#0`
#7 [const_eval_raw] const-evaluating `bug::{{constant}}#0`
#8 [const_eval] const-evaluating + checking `bug::{{constant}}#0`
#9 [typeck_tables_of] processing `bug`
#10 [typeck_item_bodies] type-checking all item bodies
#11 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
