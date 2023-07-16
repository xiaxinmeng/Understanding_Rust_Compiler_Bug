
thread 'rustc' panicked at 'assertion failed: !out_value.has_type_flags(TypeFlags::KEEP_IN_LOCAL_TCX)', src/librustc/infer/canonical/canonicalizer.rs:548:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /home/lukas/dev/rust/src/liballoc/boxed.rs:780
   7: rustc::util::common::panic_hook
             at src/librustc/util/common.rs:40
   8: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   9: std::panicking::begin_panic
             at /home/lukas/dev/rust/src/libstd/panicking.rs:411
  10: rustc::infer::canonical::canonicalizer::Canonicalizer::canonicalize
             at /home/lukas/dev/rust/<::std::macros::panic macros>:4
  11: rustc::infer::canonical::canonicalizer::<impl rustc::infer::InferCtxt>::canonicalize_response
             at /home/lukas/dev/rust/src/librustc/infer/canonical/canonicalizer.rs:93
  12: rustc::infer::canonical::query_response::<impl rustc::infer::InferCtxt>::make_query_response_ignoring_pending_obligations
             at /home/lukas/dev/rust/src/librustc/infer/canonical/query_response.rs:133
  13: rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt>::probe_op::{{closure}}
             at src/librustc_typeck/check/method/probe.rs:305
  14: rustc::infer::InferCtxt::probe
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:844
  15: rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt>::probe_op
             at src/librustc_typeck/check/method/probe.rs:288
  16: rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt>::probe_for_name
             at src/librustc_typeck/check/method/probe.rs:251
  17: rustc_typeck::check::method::<impl rustc_typeck::check::FnCtxt>::resolve_ufcs
             at src/librustc_typeck/check/method/mod.rs:433
  18: rustc_typeck::check::FnCtxt::resolve_ty_and_res_ufcs
             at src/librustc_typeck/check/mod.rs:3468
  19: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_path
             at src/librustc_typeck/check/expr.rs:448
  20: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
             at src/librustc_typeck/check/expr.rs:219
  21: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
             at src/librustc_typeck/check/expr.rs:157
  22: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
             at src/librustc_typeck/check/expr.rs:118
  23: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr
             at src/librustc_typeck/check/expr.rs:122
  24: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
             at src/librustc_typeck/check/callee.rs:44
  25: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
             at src/librustc_typeck/check/expr.rs:257
  26: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
             at src/librustc_typeck/check/expr.rs:157
  27: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
             at src/librustc_typeck/check/expr.rs:118
  28: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_hint
             at src/librustc_typeck/check/expr.rs:110
  29: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_coercable_to_type
             at src/librustc_typeck/check/expr.rs:100
  30: rustc_typeck::check::FnCtxt::check_decl_initializer
             at src/librustc_typeck/check/mod.rs:3520
  31: rustc_typeck::check::FnCtxt::check_decl_local
             at src/librustc_typeck/check/mod.rs:3529
  32: rustc_typeck::check::FnCtxt::check_stmt
             at src/librustc_typeck/check/mod.rs:3564
  33: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
             at src/librustc_typeck/check/mod.rs:3640
  34: rustc_typeck::check::FnCtxt::with_breakable_ctxt
             at src/librustc_typeck/check/mod.rs:4412
  35: rustc_typeck::check::FnCtxt::check_block_with_expected
             at src/librustc_typeck/check/mod.rs:3638
  36: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
             at src/librustc_typeck/check/expr.rs:254
  37: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
             at src/librustc_typeck/check/expr.rs:157
  38: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
             at src/librustc_typeck/check/expr.rs:118
  39: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_hint
             at src/librustc_typeck/check/expr.rs:110
  40: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
             at src/librustc_typeck/check/expr.rs:644
  41: rustc_typeck::check::check_fn
             at src/librustc_typeck/check/mod.rs:1122
  42: rustc_typeck::check::typeck_tables_of::{{closure}}
             at src/librustc_typeck/check/mod.rs:855
  43: rustc_typeck::check::InheritedBuilder::enter::{{closure}}
             at src/librustc_typeck/check/mod.rs:619
  44: rustc::infer::InferCtxtBuilder::enter::{{closure}}
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:522
  45: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1643
  46: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1866
  47: rustc::ty::context::tls::set_tlv
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1799
  48: rustc::ty::context::tls::enter_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1865
  49: rustc::ty::context::GlobalCtxt::enter_local::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1642
  50: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1972
  51: rustc::ty::context::tls::with_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1955
  52: rustc::ty::context::tls::with_context_opt
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1944
  53: rustc::ty::context::tls::with_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1955
  54: rustc::ty::context::tls::with_related_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1968
  55: rustc::ty::context::GlobalCtxt::enter_local
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1634
  56: rustc::infer::InferCtxtBuilder::enter
             at /home/lukas/dev/rust/src/librustc/infer/mod.rs:521
  57: rustc_typeck::check::InheritedBuilder::enter
             at src/librustc_typeck/check/mod.rs:619
  58: rustc_typeck::check::typeck_tables_of
             at src/librustc_typeck/check/mod.rs:834
  59: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:997
  60: rustc::ty::query::__query_compute::typeck_tables_of
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:948
  61: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:989
  62: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/lukas/dev/rust/src/librustc/dep_graph/graph.rs:334
  63: rustc::dep_graph::graph::DepGraph::with_task
             at /home/lukas/dev/rust/src/librustc/dep_graph/graph.rs:202
  64: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:556
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:275
  66: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1866
  67: rustc::ty::context::tls::set_tlv
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1799
  68: rustc::ty::context::tls::enter_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1865
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:274
  70: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1972
  71: rustc::ty::context::tls::with_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1955
  72: rustc::ty::context::tls::with_context_opt
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1944
  73: rustc::ty::context::tls::with_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1955
  74: rustc::ty::context::tls::with_related_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1968
  75: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:263
  76: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:548
  77: rustc::ty::query::plumbing::with_diagnostics
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:209
  78: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:547
  79: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:376
  80: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:610
  81: rustc::ty::query::TyCtxtEnsure::typeck_tables_of
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:1025
  82: rustc_typeck::check::typeck_item_bodies::{{closure}}
             at src/librustc_typeck/check/mod.rs:713
  83: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/mod.rs:2775
  84: core::iter::traits::iterator::Iterator::for_each::{{closure}}
             at /home/lukas/dev/rust/src/libcore/iter/traits/iterator.rs:602
  85: <core::slice::Iter<T> as core::iter::traits::iterator::Iterator>::fold
             at /home/lukas/dev/rust/src/libcore/slice/mod.rs:3187
  86: core::iter::traits::iterator::Iterator::for_each
             at /home/lukas/dev/rust/src/libcore/iter/traits/iterator.rs:602
  87: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
             at /home/lukas/dev/rust/src/librustc/ty/mod.rs:2774
  88: rustc_typeck::check::typeck_item_bodies
             at src/librustc_typeck/check/mod.rs:712
  89: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_item_bodies>::compute::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:997
  90: rustc::ty::query::__query_compute::typeck_item_bodies
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:948
  91: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /home/lukas/dev/rust/src/librustc/dep_graph/graph.rs:334
  92: rustc::dep_graph::graph::DepGraph::with_task
             at /home/lukas/dev/rust/src/librustc/dep_graph/graph.rs:202
  93: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:556
  94: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:275
  95: rustc::ty::context::tls::enter_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1866
  96: rustc::ty::context::tls::set_tlv
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1799
  97: rustc::ty::context::tls::enter_context
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1865
  98: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/query/plumbing.rs:274
  99: rustc::ty::context::tls::with_related_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1972
  100: rustc::ty::context::tls::with_context::{{closure}}
             at /home/lukas/dev/rust/src/librustc/ty/context.rs:1955
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
