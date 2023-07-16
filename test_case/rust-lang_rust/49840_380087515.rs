
error: internal compiler error: librustc/infer/region_constraints/mod.rs:695: cannot relate bound region: ReLateBound(DebruijnIndex { depth: 0 }, BrAnon(0)) <= ReStatic
  --> libcore/panicking.rs:43:1
   |
43 | / pub fn panic(expr_file_line_col: &(&'static str, &'static str, u32, u32)) -> ! {
44 | |     // Use Arguments::new_v1 instead of format_args!("{}", expr) to potentially
45 | |     // reduce size overhead. The format_args! macro uses str's Display trait to
46 | |     // write expr, which calls Formatter::pad, which must accommodate string
...  |
51 | |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]), &(file, line, col))
52 | | }
   | |_^

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:488:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::span_bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::span_bug_fmt
  14: rustc::infer::region_constraints::RegionConstraintCollector::make_subregion
  15: rustc::infer::InferCtxt::sub_regions
  16: rustc::infer::outlives::obligations::TypeOutlives::components_must_outlive
  17: rustc::infer::outlives::obligations::TypeOutlives::type_must_outlive
  18: rustc::infer::outlives::obligations::<impl rustc::infer::InferCtxt<'cx, 'gcx, 'tcx>>::process_registered_region_obligations
  19: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_item
  20: rustc::ty::context::tls::with_related_context
  21: rustc::infer::InferCtxtBuilder::enter
  22: rustc_typeck::check::wfcheck::check_item_well_formed
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc::ty::context::tls::with_related_context
  25: rustc::ty::maps::<impl rustc::ty::maps::queries::check_item_well_formed<'tcx>>::force_with_lock
  26: rustc::ty::maps::<impl rustc::ty::maps::queries::check_item_well_formed<'tcx>>::try_get
  27: rustc::ty::maps::TyCtxtAt::check_item_well_formed
  28: rustc::ty::maps::<impl rustc::ty::maps::queries::check_item_well_formed<'tcx>>::ensure
  29: rustc::hir::Crate::visit_all_item_likes
  30: rustc::util::common::time
  31: rustc_typeck::check_crate
  32: rustc::ty::context::tls::enter_context
  33: <std::thread::local::LocalKey<T>>::with
  34: rustc::ty::context::TyCtxt::create_and_enter
  35: rustc_driver::driver::compile_input
  36: rustc_driver::run_compiler_impl
  37: syntax::with_globals
  38: rustc_driver::run
  39: rustc_driver::main
  40: std::rt::lang_start::{{closure}}
  41: std::panicking::try::do_call
  42: __rust_maybe_catch_panic
  43: std::rt::lang_start_internal
  44: main
  45: __libc_start_main
  46: _start
