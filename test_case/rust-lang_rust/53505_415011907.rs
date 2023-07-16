
$ rustup show active-toolchain
nightly-x86_64-pc-windows-msvc

$ rustc --version
rustc 1.30.0-nightly (33b923fd4 2018-08-18)

$ cargo --version
cargo 1.29.0-nightly (6a7672ef5 2018-08-14)

$ RUST_BACKTRACE=1 cargo doc
 Documenting typenum v1.10.0 (file:///C:/Users/[redacted]/tmp/typenum)
error: internal compiler error: librustc\traits\structural_impls.rs:178: impossible case reached

thread '<unnamed>' panicked at 'Box<Any>', librustc_errors\lib.rs:579:9
stack backtrace:
   0: <std::sys::windows::args::Args as core::ops::drop::Drop>::drop
   1: <std::sys::windows::dynamic_lib::DynamicLibrary as core::ops::drop::Drop>::drop
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: std::panicking::rust_panic_with_hook
   5: <rustc_errors::diagnostic::SubDiagnostic as core::fmt::Debug>::fmt
   6: rustc_errors::Handler::bug
   7: rustc::util::bug::bug_fmt
   8: rustc::ty::context::tls::track_diagnostic
   9: rustc::ty::context::tls::track_diagnostic
  10: rustc::ty::context::tls::track_diagnostic
  11: rustc::util::bug::bug_fmt
  12: rustc::util::bug::bug_fmt
  13: rustc::traits::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::traits::SelectionError<'a>>::lift_to_tcx
  14: rustc::ty::context::TyCtxt::intern_layout
  15: rustc::traits::select::SelectionContext::coinductive_predicate
  16: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  17: rustc::dep_graph::graph::DepGraph::assert_ignored
  18: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  19: rustc::infer::InferCtxt::commit_from
  20: rustc::ty::context::TypeckTables::expr_ty
  21: <unknown>
  22: rustc::traits::select::SelectionContext::coinductive_predicate
  23: rustc::dep_graph::graph::DepGraph::assert_ignored
  24: rustc::traits::select::SelectionContext::coinductive_predicate
  25: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  26: rustc::dep_graph::graph::DepGraph::assert_ignored
  27: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  28: rustc::infer::InferCtxt::commit_from
  29: rustc::ty::context::TypeckTables::expr_ty
  30: <unknown>
  31: rustc::traits::select::SelectionContext::coinductive_predicate
  32: rustc::dep_graph::graph::DepGraph::assert_ignored
  33: rustc::traits::select::SelectionContext::coinductive_predicate
  34: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  35: rustc::dep_graph::graph::DepGraph::assert_ignored
  36: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  37: rustc::infer::InferCtxt::commit_from
  38: rustc::ty::context::TypeckTables::expr_ty
  39: <unknown>
  40: rustc::traits::select::SelectionContext::coinductive_predicate
  41: rustc::dep_graph::graph::DepGraph::assert_ignored
  42: rustc::traits::select::SelectionContext::coinductive_predicate
  43: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  44: rustc::dep_graph::graph::DepGraph::assert_ignored
  45: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  46: rustc::infer::InferCtxt::commit_from
  47: rustc::ty::context::TypeckTables::expr_ty
  48: <unknown>
  49: rustc::traits::select::SelectionContext::coinductive_predicate
  50: rustc::dep_graph::graph::DepGraph::assert_ignored
  51: rustc::traits::select::SelectionContext::coinductive_predicate
  52: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  53: rustc::dep_graph::graph::DepGraph::assert_ignored
  54: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  55: rustc::infer::InferCtxt::commit_from
  56: rustc::ty::context::TypeckTables::expr_ty
  57: <unknown>
  58: rustc::traits::select::SelectionContext::coinductive_predicate
  59: rustc::dep_graph::graph::DepGraph::assert_ignored
  60: rustc::traits::select::SelectionContext::coinductive_predicate
  61: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  62: rustc::dep_graph::graph::DepGraph::assert_ignored
  63: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  64: rustc::infer::InferCtxt::commit_from
  65: rustc::ty::context::TypeckTables::expr_ty
  66: <unknown>
  67: rustc::traits::select::SelectionContext::coinductive_predicate
  68: rustc::dep_graph::graph::DepGraph::assert_ignored
  69: rustc::traits::select::SelectionContext::coinductive_predicate
  70: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  71: rustc::dep_graph::graph::DepGraph::assert_ignored
  72: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  73: rustc::infer::InferCtxt::commit_from
  74: rustc::ty::context::TypeckTables::expr_ty
  75: <unknown>
  76: rustc::traits::select::SelectionContext::coinductive_predicate
  77: rustc::dep_graph::graph::DepGraph::assert_ignored
  78: rustc::traits::select::SelectionContext::coinductive_predicate
  79: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  80: rustc::dep_graph::graph::DepGraph::assert_ignored
  81: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  82: rustc::infer::InferCtxt::commit_from
  83: rustc::ty::context::TypeckTables::expr_ty
  84: <unknown>
  85: rustc::traits::select::SelectionContext::coinductive_predicate
  86: rustc::dep_graph::graph::DepGraph::assert_ignored
  87: rustc::traits::select::SelectionContext::coinductive_predicate
  88: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  89: rustc::dep_graph::graph::DepGraph::assert_ignored
  90: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  91: rustc::infer::InferCtxt::commit_from
  92: rustc::ty::context::TypeckTables::expr_ty
  93: <unknown>
  94: rustc::traits::select::SelectionContext::coinductive_predicate
  95: rustc::dep_graph::graph::DepGraph::assert_ignored
  96: rustc::traits::select::SelectionContext::coinductive_predicate
  97: rustc::traits::select::SelectionContext::evaluate_obligation_recursively
  98: rustc::dep_graph::graph::DepGraph::assert_ignored
  99: rustc::traits::select::SelectionContext::evaluate_obligation_recursively

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.30.0-nightly (33b923fd4 2018-08-18) running on x86_64-pc-windows-msvc

error: Could not document `typenum`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name typenum 'src\lib.rs' -o 'C:\Users\[redacted]\tmp\typenum\target\doc' -L 'dependency=C:\Users\[redacted]\tmp\typenum\target\debug\deps'` (exit code: 1)

