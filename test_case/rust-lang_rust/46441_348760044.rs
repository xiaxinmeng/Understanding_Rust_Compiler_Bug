
note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: def_id.is_local()', src/librustc/hir/map/mod.rs:309:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc::hir::map::Map::def_key
   7: <rustc::middle::resolve_lifetime::LifetimeContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_path
   8: <rustc::middle::resolve_lifetime::LifetimeContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_ty
   9: <rustc::middle::resolve_lifetime::LifetimeContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_impl_item
  10: rustc::hir::intravisit::walk_impl_item_ref
  11: rustc::hir::intravisit::walk_item
  12: <rustc::middle::resolve_lifetime::LifetimeContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  13: rustc::session::Session::track_errors
  14: rustc::middle::resolve_lifetime::resolve_lifetimes
  15: rustc::ty::maps::<impl rustc::ty::maps::queries::resolve_lifetimes<'tcx>>::compute_result
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc::dep_graph::graph::DepGraph::with_task
  18: rustc_errors::Handler::track_diagnostics
  19: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  20: rustc::ty::maps::<impl rustc::ty::maps::queries::resolve_lifetimes<'tcx>>::force
  21: rustc::ty::maps::<impl rustc::ty::maps::queries::resolve_lifetimes<'tcx>>::try_get
  22: rustc::ty::maps::TyCtxtAt::resolve_lifetimes
  23: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::resolve_lifetimes
  24: core::ops::function::FnOnce::call_once
  25: rustc::ty::maps::<impl rustc::ty::maps::queries::object_lifetime_defaults_map<'tcx>>::compute_result
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::dep_graph::graph::DepGraph::with_task
  28: rustc_errors::Handler::track_diagnostics
  29: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  30: rustc::ty::maps::<impl rustc::ty::maps::queries::object_lifetime_defaults_map<'tcx>>::force
  31: rustc::ty::maps::<impl rustc::ty::maps::queries::object_lifetime_defaults_map<'tcx>>::try_get
  32: rustc::ty::maps::TyCtxtAt::object_lifetime_defaults_map
  33: rustc::ty::context::TyCtxt::object_lifetime_defaults
  34: rustc_typeck::collect::generics_of
  35: rustc::ty::maps::<impl rustc::ty::maps::queries::generics_of<'tcx>>::compute_result
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::dep_graph::graph::DepGraph::with_task
  38: rustc_errors::Handler::track_diagnostics
  39: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  40: rustc::ty::maps::<impl rustc::ty::maps::queries::generics_of<'tcx>>::force
  41: rustc::ty::maps::<impl rustc::ty::maps::queries::generics_of<'tcx>>::try_get
  42: rustc::ty::maps::TyCtxtAt::generics_of
  43: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::generics_of
  44: <rustc_typeck::collect::CollectItemTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  45: rustc_typeck::collect::collect_item_types
  46: rustc_typeck::check_crate
  47: <std::thread::local::LocalKey<T>>::with
  48: rustc::ty::context::tls::enter
  49: <std::thread::local::LocalKey<T>>::with
  50: rustc::ty::context::TyCtxt::create_and_enter
  51: rustc_driver::driver::compile_input
  52: rustc_driver::run_compiler

error: Could not compile `alloc`.

Caused by:
  process didn't exit successfully: `/Users/gkholkar/Desktop/or/rust/build/bootstrap/debug/rustc --crate-name alloc src/liballoc/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=22e63b537c61a4f8 -C extra-filename=-22e63b537c61a4f8 --out-dir /Users/gkholkar/Desktop/or/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps --target x86_64-apple-darwin -L dependency=/Users/gkholkar/Desktop/or/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps -L dependency=/Users/gkholkar/Desktop/or/rust/build/x86_64-apple-darwin/stage1-std/release/deps --extern std_unicode=/Users/gkholkar/Desktop/or/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libstd_unicode-aa82a4fba39f1481.rlib --extern core=/Users/gkholkar/Desktop/or/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps/libcore-a1b407ed3953731d.rlib` (exit code: 101)
thread 'main' panicked at 'command did not execute successfully: "/Users/gkholkar/Desktop/or/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/Users/gkholkar/Desktop/or/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', src/bootstrap/compile.rs:882:8
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: std::panicking::begin_panic_fmt
   7: bootstrap::compile::run_cargo
   8: <bootstrap::compile::Std as bootstrap::builder::Step>::run
   9: bootstrap::builder::Builder::ensure
  10: <bootstrap::compile::Test as bootstrap::builder::Step>::run
  11: bootstrap::builder::Builder::ensure
  12: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
  13: bootstrap::builder::Builder::ensure
  14: <bootstrap::compile::Rustc as bootstrap::builder::Step>::make_run
  15: bootstrap::builder::StepDescription::maybe_run
  16: bootstrap::builder::StepDescription::run
  17: bootstrap::builder::Builder::run
  18: bootstrap::Build::build
  19: bootstrap::main
  20: __rust_maybe_catch_panic
  21: std::rt
