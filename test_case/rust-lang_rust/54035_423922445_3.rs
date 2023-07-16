
error: internal compiler error: librustc/traits/structural_impls.rs:178: impossible case reached

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:578:9
error: internal compiler error: librustc/traits/structural_impls.rs:178: impossible case reached

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:578:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::bug_fmt
  14: rustc::traits::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::traits::SelectionError<'a>>::lift_to_tcx
  15: rustc::ty::context::TyCtxt::lift_to_global
  16: rustc::traits::select::SelectionContext::candidate_from_obligation
  17: rustc::traits::select::SelectionContext::evaluate_stack
  18: rustc::ty::context::tls::with_context
  19: rustc::dep_graph::graph::DepGraph::with_anon_task
  20: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  21: rustc::infer::InferCtxt::probe
  22: <&'a mut I as core::iter::iterator::Iterator>::next
  23: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  24: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  25: rustc::ty::context::tls::with_context
  26: rustc::dep_graph::graph::DepGraph::with_anon_task
  27: rustc::traits::select::SelectionContext::candidate_from_obligation
  28: rustc::traits::select::SelectionContext::evaluate_stack
  29: rustc::ty::context::tls::with_context
  30: rustc::dep_graph::graph::DepGraph::with_anon_task
  31: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  32: rustc::infer::InferCtxt::probe
  33: <&'a mut I as core::iter::iterator::Iterator>::next
  34: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  35: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  36: rustc::ty::context::tls::with_context
  37: rustc::dep_graph::graph::DepGraph::with_anon_task
  38: rustc::traits::select::SelectionContext::candidate_from_obligation
  39: rustc::traits::select::SelectionContext::evaluate_stack
  40: rustc::ty::context::tls::with_context
  41: rustc::dep_graph::graph::DepGraph::with_anon_task
  42: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  43: rustc::infer::InferCtxt::probe
  44: <&'a mut I as core::iter::iterator::Iterator>::next
  45: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  46: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  47: rustc::ty::context::tls::with_context
  48: rustc::dep_graph::graph::DepGraph::with_anon_task
  49: rustc::traits::select::SelectionContext::candidate_from_obligation
  50: rustc::traits::select::SelectionContext::evaluate_stack
  51: rustc::ty::context::tls::with_context
  52: rustc::dep_graph::graph::DepGraph::with_anon_task
  53: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  54: rustc::infer::InferCtxt::probe
  55: <&'a mut I as core::iter::iterator::Iterator>::next
  56: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  57: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  58: rustc::ty::context::tls::with_context
  59: rustc::dep_graph::graph::DepGraph::with_anon_task
  60: rustc::traits::select::SelectionContext::candidate_from_obligation
  61: rustc::traits::select::SelectionContext::evaluate_stack
  62: rustc::ty::context::tls::with_context
  63: rustc::dep_graph::graph::DepGraph::with_anon_task
  64: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  65: rustc::infer::InferCtxt::probe
  66: <&'a mut I as core::iter::iterator::Iterator>::next
  67: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  68: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  69: rustc::ty::context::tls::with_context
  70: rustc::dep_graph::graph::DepGraph::with_anon_task
  71: rustc::traits::select::SelectionContext::candidate_from_obligation
  72: rustc::traits::select::SelectionContext::evaluate_stack
  73: rustc::ty::context::tls::with_context
  74: rustc::dep_graph::graph::DepGraph::with_anon_task
  75: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  76: rustc::infer::InferCtxt::probe
  77: <&'a mut I as core::iter::iterator::Iterator>::next
  78: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  79: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  80: rustc::ty::context::tls::with_context
  81: rustc::dep_graph::graph::DepGraph::with_anon_task
  82: rustc::traits::select::SelectionContext::candidate_from_obligation
  83: rustc::traits::select::SelectionContext::evaluate_stack
  84: rustc::ty::context::tls::with_context
  85: rustc::dep_graph::graph::DepGraph::with_anon_task
  86: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  87: rustc::infer::InferCtxt::probe
  88: <&'a mut I as core::iter::iterator::Iterator>::next
  89: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  90: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  91: rustc::ty::context::tls::with_context
  92: rustc::dep_graph::graph::DepGraph::with_anon_task
  93: rustc::traits::select::SelectionContext::candidate_from_obligation
  94: rustc::traits::select::SelectionContext::evaluate_stack
  95: rustc::ty::context::tls::with_context
  96: rustc::dep_graph::graph::DepGraph::with_anon_task
  97: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  98: rustc::infer::InferCtxt::probe
  99: <&'a mut I as core::iter::iterator::Iterator>::next
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `&_: std::io::Write`
#1 [typeck_tables_of] processing `core::watcher::project::Project::new`
#2 [typeck_item_bodies] type-checking all item bodies
end of query stack
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::bug_fmt
  14: rustc::traits::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::traits::SelectionError<'a>>::lift_to_tcx
  15: rustc::ty::context::TyCtxt::lift_to_global
  16: rustc::traits::select::SelectionContext::candidate_from_obligation
  17: rustc::traits::select::SelectionContext::evaluate_stack
  18: rustc::ty::context::tls::with_context
  19: rustc::dep_graph::graph::DepGraph::with_anon_task
  20: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  21: rustc::infer::InferCtxt::probe
  22: <&'a mut I as core::iter::iterator::Iterator>::next
  23: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  24: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  25: rustc::ty::context::tls::with_context
  26: rustc::dep_graph::graph::DepGraph::with_anon_task
  27: rustc::traits::select::SelectionContext::candidate_from_obligation
  28: rustc::traits::select::SelectionContext::evaluate_stack
  29: rustc::ty::context::tls::with_context
  30: rustc::dep_graph::graph::DepGraph::with_anon_task
  31: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  32: rustc::infer::InferCtxt::probe
  33: <&'a mut I as core::iter::iterator::Iterator>::next
  34: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  35: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  36: rustc::ty::context::tls::with_context
  37: rustc::dep_graph::graph::DepGraph::with_anon_task
  38: rustc::traits::select::SelectionContext::candidate_from_obligation
  39: rustc::traits::select::SelectionContext::evaluate_stack
  40: rustc::ty::context::tls::with_context
  41: rustc::dep_graph::graph::DepGraph::with_anon_task
  42: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  43: rustc::infer::InferCtxt::probe
  44: <&'a mut I as core::iter::iterator::Iterator>::next
  45: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  46: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  47: rustc::ty::context::tls::with_context
  48: rustc::dep_graph::graph::DepGraph::with_anon_task
  49: rustc::traits::select::SelectionContext::candidate_from_obligation
  50: rustc::traits::select::SelectionContext::evaluate_stack
  51: rustc::ty::context::tls::with_context
  52: rustc::dep_graph::graph::DepGraph::with_anon_task
  53: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  54: rustc::infer::InferCtxt::probe
  55: <&'a mut I as core::iter::iterator::Iterator>::next
  56: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  57: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  58: rustc::ty::context::tls::with_context
  59: rustc::dep_graph::graph::DepGraph::with_anon_task
  60: rustc::traits::select::SelectionContext::candidate_from_obligation
  61: rustc::traits::select::SelectionContext::evaluate_stack
  62: rustc::ty::context::tls::with_context
  63: rustc::dep_graph::graph::DepGraph::with_anon_task
  64: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  65: rustc::infer::InferCtxt::probe
  66: <&'a mut I as core::iter::iterator::Iterator>::next
  67: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  68: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  69: rustc::ty::context::tls::with_context
  70: rustc::dep_graph::graph::DepGraph::with_anon_task
  71: rustc::traits::select::SelectionContext::candidate_from_obligation
  72: rustc::traits::select::SelectionContext::evaluate_stack
  73: rustc::ty::context::tls::with_context
  74: rustc::dep_graph::graph::DepGraph::with_anon_task
  75: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  76: rustc::infer::InferCtxt::probe
  77: <&'a mut I as core::iter::iterator::Iterator>::next
  78: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  79: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  80: rustc::ty::context::tls::with_context
  81: rustc::dep_graph::graph::DepGraph::with_anon_task
  82: rustc::traits::select::SelectionContext::candidate_from_obligation
  83: rustc::traits::select::SelectionContext::evaluate_stack
  84: rustc::ty::context::tls::with_context
  85: rustc::dep_graph::graph::DepGraph::with_anon_task
  86: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  87: rustc::infer::InferCtxt::probe
  88: <&'a mut I as core::iter::iterator::Iterator>::next
  89: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  90: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  91: rustc::ty::context::tls::with_context
  92: rustc::dep_graph::graph::DepGraph::with_anon_task
  93: rustc::traits::select::SelectionContext::candidate_from_obligation
  94: rustc::traits::select::SelectionContext::evaluate_stack
  95: rustc::ty::context::tls::with_context
  96: rustc::dep_graph::graph::DepGraph::with_anon_task
  97: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  98: rustc::infer::InferCtxt::probe
  99: <&'a mut I as core::iter::iterator::Iterator>::next
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `&_: std::io::Write`
#1 [typeck_tables_of] processing `core::watcher::project::Project::new`
#2 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error

error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.29.0 (aa3ca1994 2018-09-11) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.29.0 (aa3ca1994 2018-09-11) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden
