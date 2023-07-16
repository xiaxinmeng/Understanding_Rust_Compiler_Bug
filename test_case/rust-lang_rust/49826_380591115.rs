
$ RUST_BACKTRACE=1 rustc +dev2 -Z continue-parse-after-error src/test/compile-fail/issue-36638.rs
error: expected identifier, found keyword `Self`
  --> src/test/compile-fail/issue-36638.rs:13:12
   |
13 | struct Foo<Self>(Self);
   |            ^^^^ expected identifier, found keyword

error: expected identifier, found keyword `Self`
  --> src/test/compile-fail/issue-36638.rs:16:11
   |
16 | trait Bar<Self> {}
   |           ^^^^ expected identifier, found keyword

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', librustc/ty/sty.rs:889:13
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: rustc::ty::sty::ParamTy::is_self
   8: rustc::ty::flags::FlagComputation::for_sty
   9: rustc::ty::context::CtxtInterners::intern_ty
  10: rustc::ty::subst::<impl rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::fill_item
  11: rustc::ty::subst::<impl rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::identity_for_item
  12: rustc_typeck::collect::predicates_of
  13: rustc::ty::maps::<impl rustc::ty::maps::queries::predicates_of<'tcx>>::compute_result
  14: rustc::dep_graph::graph::DepGraph::with_task_impl
  15: rustc::ty::context::tls::with_related_context
  16: rustc::ty::maps::<impl rustc::ty::maps::queries::predicates_of<'tcx>>::force_with_lock
  17: rustc::ty::maps::<impl rustc::ty::maps::queries::predicates_of<'tcx>>::try_get
  18: rustc::ty::maps::TyCtxtAt::predicates_of
  19: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::predicates_of
  20: <rustc_typeck::collect::CollectItemTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  21: rustc::hir::Crate::visit_all_item_likes
  22: rustc::session::Session::track_errors
  23: rustc_typeck::check_crate
  24: rustc::ty::context::tls::enter_context
  25: <std::thread::local::LocalKey<T>>::with
  26: rustc::ty::context::TyCtxt::create_and_enter
  27: rustc_driver::driver::compile_input
  28: rustc_driver::run_compiler_impl
  29: syntax::with_globals
  30: rustc_driver::main
  31: std::rt::lang_start::{{closure}}
  32: std::panicking::try::do_call
  33: __rust_maybe_catch_panic
  34: std::rt::lang_start_internal
  35: main
  36: __libc_start_main
  37: _start
query stack during panic:
#0 [predicates_of] processing `Bar`
  --> src/test/compile-fail/issue-36638.rs:16:1
   |
16 | trait Bar<Self> {}
   | ^^^^^^^^^^^^^^^
end of query stack
error: aborting due to 2 previous errors

