
error: internal compiler error: src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:588:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::hir::def::Def::def_id::{{closure}}
  15: rustc::hir::def::Def::def_id
  16: rustc_typeck::check::method::suggest::compute_all_traits::handle_external_def
  17: rustc_typeck::check::method::suggest::compute_all_traits::handle_external_def
  18: rustc_typeck::check::method::suggest::compute_all_traits::handle_external_def
  19: core::ops::function::FnOnce::call_once
  20: rustc::ty::query::__query_compute::all_traits
  21: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::all_traits<'tcx>>::compute
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  24: rustc_typeck::check::method::suggest::all_traits
  25: rustc_typeck::check::method::probe::ProbeContext::assemble_extension_candidates_for_all_traits
  26: rustc::infer::InferCtxt::probe
  27: rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::probe_for_return_type
  28: rustc_typeck::check::FnCtxt::suggest_ref_or_into
  29: rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::demand_coerce_diag
  30: rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::demand_coerce
  31: rustc_typeck::check::FnCtxt::check_argument_types
  32: rustc_typeck::check::FnCtxt::check_method_argument_types
  33: rustc_typeck::check::FnCtxt::check_expr_kind
  34: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  35: rustc_typeck::check::FnCtxt::check_expr_kind
  36: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  37: rustc_typeck::check::FnCtxt::check_decl_initializer
  38: rustc_typeck::check::FnCtxt::check_decl_local
  39: rustc_typeck::check::FnCtxt::check_stmt
  40: rustc_typeck::check::FnCtxt::check_block_with_expected
  41: rustc_typeck::check::FnCtxt::check_expr_kind
  42: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  43: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_match
  44: rustc_typeck::check::FnCtxt::check_expr_kind
  45: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  46: rustc_typeck::check::FnCtxt::check_return_expr
  47: rustc_typeck::check::check_fn
  48: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_expr_closure
  49: rustc_typeck::check::FnCtxt::check_expr_kind
  50: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  51: rustc_typeck::check::FnCtxt::check_argument_types
  52: rustc_typeck::check::FnCtxt::check_method_argument_types
  53: rustc_typeck::check::FnCtxt::check_expr_kind
  54: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  55: rustc_typeck::check::FnCtxt::check_stmt
  56: rustc_typeck::check::FnCtxt::check_block_with_expected
  57: rustc_typeck::check::FnCtxt::check_expr_kind
  58: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  59: rustc_typeck::check::FnCtxt::check_return_expr
  60: rustc_typeck::check::check_fn
  61: rustc::ty::context::GlobalCtxt::enter_local
  62: rustc_typeck::check::typeck_tables_of
  63: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  64: rustc::dep_graph::graph::DepGraph::with_task_impl
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  66: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  67: rustc::session::Session::track_errors
  68: rustc_typeck::check::typeck_item_bodies
  69: rustc::ty::query::__query_compute::typeck_item_bodies
  70: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  71: rustc::dep_graph::graph::DepGraph::with_task_impl
  72: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  73: rustc::util::common::time
  74: rustc_typeck::check_crate
  75: <std::thread::local::LocalKey<T>>::with
  76: rustc::ty::context::TyCtxt::create_and_enter
  77: rustc_driver::driver::compile_input
  78: rustc_driver::run_compiler_with_pool
  79: <scoped_tls::ScopedKey<T>>::set
  80: rustc_driver::run_compiler
  81: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [all_traits] fetching all foreign and local traits
#1 [typeck_tables_of] processing `resolve_imports::ImportResolver::finalize_import`
#2 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-beta.1 (d1add9723 2019-01-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=n -C codegen-units=4 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `rustc_resolve`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/pair/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--features" "" "--manifest-path" "/home/pair/rust/src/rustc/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
