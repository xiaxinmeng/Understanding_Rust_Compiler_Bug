
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real("src/main.rs"), BytePos(0)), end: (Macros("::async_stream::stream"), BytePos(12713142)) })', src/libcore/result.rs:1165:5
stack backtrace:
  <snip>
  15: core::result::unwrap_failed
             at src/libcore/result.rs:1165
  16: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  17: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  18: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
  19: rustc_errors::HandlerInner::emit_diagnostic
  20: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  21: rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt>::demand_coerce
  22: rustc_typeck::check::FnCtxt::check_argument_types
  23: rustc_typeck::check::FnCtxt::check_method_argument_types
  24: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  25: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  26: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_meets_expectation_or_error
  27: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  28: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  29: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  30: rustc_typeck::check::FnCtxt::check_stmt
  31: rustc_typeck::check::FnCtxt::check_block_with_expected
  32: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  33: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  34: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_meets_expectation_or_error
  35: rustc_typeck::check::FnCtxt::check_stmt
  36: rustc_typeck::check::FnCtxt::check_block_with_expected
  37: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  38: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  39: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  40: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  41: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  42: rustc_typeck::check::FnCtxt::check_block_with_expected
  43: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  44: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  45: rustc_typeck::check::FnCtxt::check_block_with_expected
  46: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  47: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  48: rustc_typeck::check::FnCtxt::check_block_with_expected
  49: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  50: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  51: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  52: rustc_typeck::check::check_fn
  53: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  54: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  55: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  56: rustc_typeck::check::FnCtxt::check_argument_types
  57: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  58: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  59: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  60: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  61: rustc_typeck::check::FnCtxt::check_argument_types
  62: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  63: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  64: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  65: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  66: rustc_typeck::check::FnCtxt::check_block_with_expected
  67: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  68: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  69: rustc_typeck::check::FnCtxt::check_decl_initializer
  70: rustc_typeck::check::FnCtxt::check_decl_local
  71: rustc_typeck::check::FnCtxt::check_stmt
  72: rustc_typeck::check::FnCtxt::check_block_with_expected
  73: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  74: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  75: rustc_typeck::check::FnCtxt::check_block_with_expected
  76: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  77: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  78: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  79: rustc_typeck::check::check_fn
  80: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  81: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  82: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  83: rustc_typeck::check::FnCtxt::check_argument_types
  84: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  85: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  86: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  87: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  88: rustc_typeck::check::FnCtxt::check_argument_types
  89: rustc_typeck::check::FnCtxt::check_method_argument_types
  90: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  91: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  92: rustc_typeck::check::FnCtxt::check_block_with_expected
  93: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  94: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  95: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  96: rustc_typeck::check::check_fn
  97: rustc::ty::context::GlobalCtxt::enter_local
  98: rustc_typeck::check::typeck_tables_of
  99: rustc::ty::query::__query_compute::typeck_tables_of
 100: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
 101: rustc::dep_graph::graph::DepGraph::with_task_impl
 102: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
 103: rustc::ty::query::__query_compute::typeck_tables_of
 104: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
 105: rustc::dep_graph::graph::DepGraph::with_task_impl
