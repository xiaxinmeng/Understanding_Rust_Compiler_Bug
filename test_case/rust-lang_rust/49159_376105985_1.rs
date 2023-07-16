
   Compiling ice_demo v0.1.0 (file:///C:/temp/ice_demo)
     Running `rustc --crate-name ice_demo src\lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=02ad0d1ab1100592 -C extra-filename=-02ad0d1ab1100592 --out-dir C:\temp\ice_demo\target\debug\deps -C incremental=C:\temp\ice_demo\target\debug\incremental -L dependency=C:\temp\ice_demo\target\debug\deps`
error[E0425]: cannot find function `zzz` in this scope
 --> src\lib.rs:2:5
  |
2 |     zzz(2)
  |     ^^^ not found in this scope

error: internal compiler error: librustc\ich\impls_ty.rs:906: ty::TypeVariants::hash_stable() - Unexpected variant TyInfer(?0).

thread 'rustc' panicked at 'Box<Any>', librustc_errors\lib.rs:543:9
stack backtrace:
   0: <std::sync::mpsc::select::Select as core::fmt::Debug>::fmt
   1: <std::stdsimd::arch::detect::linux::CpuInfoField<'a> as core::fmt::Debug>::fmt
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for syntax::ast::FloatTy>::lift_to_tcx
   5: std::panicking::rust_panic_with_hook
   6: <rustc_errors::lock::acquire_global_lock::Guard as core::ops::drop::Drop>::drop
   7: rustc_errors::Handler::bug
   8: rustc::session::config::nightly_options::check_nightly_options
   9: <&'a rustc::ty::Slice<rustc::ty::subst::Kind<'a>> as rustc::ty::context::Lift<'tcx>>::lift_to_tcx
  10: <&'a rustc::ty::Slice<rustc::ty::subst::Kind<'a>> as rustc::ty::context::Lift<'tcx>>::lift_to_tcx
  11: <rustc::util::common::ProfileQueriesMsg as core::fmt::Debug>::fmt
  12: <rustc::util::common::ProfileQueriesMsg as core::fmt::Debug>::fmt
  13: rustc::ty::context::tls::span_debug
  14: rustc::ty::context::tls::span_debug
  15: rustc::session::bug_fmt
  16: rustc::session::bug_fmt
  17: rustc::hir::map::describe_def
  18: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::hir::InlineAsm>::lift_to_tcx
  19: rustc::dep_graph::dep_node::DepNode::new
  20: rustc::ty::maps::<impl rustc::ty::maps::queries::dropck_outlives<'tcx>>::try_get
  21: rustc::ty::maps::TyCtxtAt::dropck_outlives
  22: rustc::traits::query::dropck_outlives::<impl rustc::infer::at::At<'cx, 'gcx, 'tcx>>::dropck_outlives
  23: <rustc_typeck::variance::test::VarianceTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  24: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  25: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  26: <rustc_typeck::check::Diverges as core::fmt::Debug>::fmt
  27: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  28: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  29: <rustc_typeck::variance::terms::InferredIndex as core::fmt::Debug>::fmt
  30: <unknown>
  31: <rustc_typeck::variance::test::VarianceTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  32: <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_impl_item
  33: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  34: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
  35: rustc::dep_graph::graph::DepGraph::assert_ignored
  36: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::hir::InlineAsm>::lift_to_tcx
  37: <rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx> as rustc::ty::layout::HasTyCtxt<'gcx>>::tcx
  38: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
  39: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  40: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  41: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
  42: <rustc_typeck::constrained_type_params::Parameter as core::fmt::Debug>::fmt
  43: <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item
  44: rustc::dep_graph::graph::DepGraph::assert_ignored
  45: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::hir::InlineAsm>::lift_to_tcx
  46: <rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx> as rustc::ty::layout::HasTyCtxt<'gcx>>::tcx
  47: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::ensure
  48: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  49: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  50: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  51: rustc_typeck::check_crate
  52: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  53: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  54: <rustc_driver::profile::trace::Query as core::fmt::Debug>::fmt
  55: rustc_driver::driver::compile_input
  56: rustc_driver::run_compiler
  57: rustc_driver::driver::build_output_filenames
  58: <unknown>
  59: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  60: _rust_maybe_catch_panic
  61: rustc_driver::driver::build_output_filenames
  62: std::process::id
  63: std::sys::windows::thread::Thread::new
  64: BaseThreadInitThunk
  65: RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (5508b2714 2018-03-18) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `ice_demo`.

Caused by:
  process didn't exit successfully: `rustc --crate-name ice_demo src\lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=02ad0d1ab1100592 -C extra-filename=-02ad0d1ab1100592 --out-dir C:\temp\ice_demo\target\debug\deps -C incremental=C:\temp\ice_demo\target\debug\incremental -L dependency=C:\temp\ice_demo\target\debug\deps` (exit code: 101)
