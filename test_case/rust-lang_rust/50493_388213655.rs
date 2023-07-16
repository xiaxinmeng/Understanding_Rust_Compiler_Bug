console
$ rustc --version && RUST_BACKTRACE=1 ./repro.sh 
rustc 1.27.0-nightly (e5f80f2a4 2018-05-09)
     Created library `repro` project
     Created library `repro_derive` project
   Compiling repro_derive v0.1.0
   Compiling repro v0.1.0
error: visibilities can only be restricted to ancestor modules
 --> src/lib.rs:7:12
  |
7 |     pub(in restricted) field: usize,
  |            ^^^^^^^^^^

error[E0616]: field `field` of struct `Restricted` is private
 --> src/lib.rs:5:10
  |
5 | #[derive(Derive)]
  |          ^^^^^^

thread 'main' panicked at 'no index for a field', libcore/option.rs:914:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:467
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:350
   7: rust_begin_unwind
             at libstd/panicking.rs:328
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:71
   9: core::option::expect_failed
             at libcore/option.rs:914
  10: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::field_index
  11: rustc::middle::mem_categorization::MemCategorizationContext::cat_expr_unadjusted
  12: rustc_typeck::check::regionck::RegionCtxt::constrain_adjustments
  13: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  14: rustc::hir::intravisit::walk_block
  15: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  16: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  17: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_fn
  18: rustc::ty::context::tls::with_related_context
  19: rustc::infer::InferCtxtBuilder::enter
  20: rustc_typeck::check::typeck_tables_of
  21: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::typeck_tables_of<'tcx>>::compute
  22: rustc::ty::context::tls::with_context
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc::ty::context::tls::with_related_context
  25: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  26: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  27: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  28: rustc::session::Session::track_errors
  29: rustc_typeck::check::typeck_item_bodies
  30: rustc::ty::context::tls::with_context
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::context::tls::with_related_context
  33: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  34: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  35: rustc_typeck::check_crate
  36: rustc::ty::context::tls::enter_context
  37: <std::thread::local::LocalKey<T>>::with
  38: rustc::ty::context::TyCtxt::create_and_enter
  39: rustc_driver::driver::compile_input
  40: rustc_driver::run_compiler_impl
  41: <scoped_tls::ScopedKey<T>>::set
  42: syntax::with_globals
  43: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  44: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  45: rustc_driver::run
  46: rustc_driver::main
  47: std::rt::lang_start::{{closure}}
  48: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  49: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  50: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:374
             at libstd/rt.rs:58
  51: main
  52: __libc_start_main
  53: <unknown>
query stack during panic:
#0 [typeck_tables_of] processing `two`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors
