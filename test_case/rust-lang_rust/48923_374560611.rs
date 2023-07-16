
thread 'rustc' panicked at 'assertion failed: !tcx.dep_graph.dep_node_exists(&dep_node)', librustc/ty/maps/mod.rs:93:1
stack backtrace:
   0:     0x7f1997ac0a0b - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h31989477674ed8eb
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f1997acd3a8 - std::sys_common::backtrace::print::h0aff9c2db7035d67
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7f1997aac41d - std::panicking::default_hook::{{closure}}::h4782d445b75d926a
                               at libstd/panicking.rs:207
   3:     0x7f1997aac166 - std::panicking::default_hook::hdb4d35de831f50b6
                               at libstd/panicking.rs:223
   4:     0x7f1993b47920 - core::ops::function::Fn::call::h3f055c710819ffad
                               at librustc/util/common.rs:51
                               at /home/mw/0-rust/src/libcore/ops/function.rs:73
   5:     0x7f1997aacafa - std::panicking::rust_panic_with_hook::h03efd923e4481138
                               at libstd/panicking.rs:403
   6:     0x7f199402de87 - std::panicking::begin_panic::h2e93ee4e51787726
                               at /home/mw/0-rust/src/libstd/panicking.rs:365
   7:     0x7f199401c671 - rustc::ty::maps::<impl rustc::ty::maps::queries::dropck_outlives<'tcx>>::force::hcf588a057e7dfc15
                               at librustc/ty/maps/plumbing.rs:494
   8:     0x7f199401d106 - rustc::ty::maps::<impl rustc::ty::maps::queries::dropck_outlives<'tcx>>::try_get::h6d4b447907112cb7
                               at librustc/ty/maps/plumbing.rs:361
                               at librustc/ty/maps/plumbing.rs:539
   9:     0x7f1993b26559 - rustc::ty::maps::TyCtxtAt::dropck_outlives::hc79eae021e5ad3f1
                               at librustc/ty/maps/plumbing.rs:578
  10:     0x7f1993efd0c1 - rustc::traits::query::dropck_outlives::<impl rustc::infer::at::At<'cx, 'gcx, 'tcx>>::dropck_outlives::haa211ead3f1ef3fd
                               at librustc/ty/maps/plumbing.rs:571
                               at librustc/traits/query/dropck_outlives.rs:55
  11:     0x7f19962a4d38 - rustc_typeck::check::dropck::check_safety_of_destructor_if_necessary::h16fa59e7c5632e1e
  12:     0x7f1996379747 - rustc_typeck::check::regionck::RegionCtxt::check_safety_of_rvalue_destructor_if_necessary::h289ec60ae2053c50
  13:     0x7f1996375480 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::ha2234207548bccea
  14:     0x7f1996374430 - rustc_typeck::check::regionck::RegionCtxt::visit_fn_body::he60f80517525fac9
  15:     0x7f19962276fd - rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_fn::h1c56e7bc971b7493
  16:     0x7f1996288e8a - rustc::ty::context::tls::enter::he1549a7dbb565f38
  17:     0x7f19961f7f07 - rustc::infer::InferCtxtBuilder::enter::h31dba36dd2a59536
  18:     0x7f19962421e1 - rustc_typeck::check::typeck_tables_of::hf5b1548ebcfe3766
  19:     0x7f1993f63bbb - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::compute_result::h181c3bba84dd23e7
                               at librustc/ty/maps/plumbing.rs:396
  20:     0x7f1993a0c9c0 - rustc::dep_graph::graph::DepGraph::with_task_impl::h7024f65b1611aca8
                               at librustc/dep_graph/graph.rs:230
  21:     0x7f19940f61f7 - rustc_errors::Handler::track_diagnostics::h854d94ebfa19a0bb
                               at librustc/dep_graph/graph.rs:200
                               at librustc/ty/maps/plumbing.rs:505
                               at /home/mw/0-rust/src/librustc_errors/lib.rs:637
  22:     0x7f1993c4a3f6 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h8bc4420f6950773d
                               at librustc/ty/maps/plumbing.rs:498
                               at librustc/ty/maps/plumbing.rs:121
  23:     0x7f1993f63cd3 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force::h7863739a808a43c4
                               at librustc/ty/maps/plumbing.rs:497
  24:     0x7f1993f64742 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get::he75b775638ad6435
                               at librustc/ty/maps/plumbing.rs:361
                               at librustc/ty/maps/plumbing.rs:539
  25:     0x7f1993b1dc00 - rustc::ty::maps::TyCtxtAt::typeck_tables_of::h68de7e885d32e066
                               at librustc/ty/maps/plumbing.rs:578
  26:     0x7f1993f63b40 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::h78aac60f3b569d09
                               at librustc/ty/maps/plumbing.rs:571
                               at librustc/ty/maps/plumbing.rs:390
  27:     0x7f1996368bb1 - rustc::session::Session::track_errors::h90088f2022f85233
  28:     0x7f1996241d8e - rustc_typeck::check::typeck_item_bodies::hb05f767c92c1fa54
  29:     0x7f1993a2f7b0 - rustc::dep_graph::graph::DepGraph::with_task_impl::hcea9f4d39a5bcfd5
                               at librustc/dep_graph/graph.rs:230
  30:     0x7f19940e5313 - rustc_errors::Handler::track_diagnostics::h2e26319d038e4464
                               at librustc/dep_graph/graph.rs:200
                               at librustc/ty/maps/plumbing.rs:505
                               at /home/mw/0-rust/src/librustc_errors/lib.rs:637
  31:     0x7f1993c309a6 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h572b8968938fa16b
                               at librustc/ty/maps/plumbing.rs:498
                               at librustc/ty/maps/plumbing.rs:121
  32:     0x7f1993f622bc - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force::h91e81b1d86c71b44
                               at librustc/ty/maps/plumbing.rs:497
  33:     0x7f1993f62c7a - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get::h4921611a8a2d333a
                               at librustc/ty/maps/plumbing.rs:361
                               at librustc/ty/maps/plumbing.rs:539
  34:     0x7f1993b1dad9 - rustc::ty::maps::TyCtxtAt::typeck_item_bodies::hb8608ceb14741f12
                               at librustc/ty/maps/plumbing.rs:578
  35:     0x7f1993c9c551 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies::h750620761ac8531b
                               at librustc/ty/maps/plumbing.rs:571
  36:     0x7f199639172e - rustc_typeck::check_crate::h818efe4efd2e8228
  37:     0x7f1997e98382 - rustc::ty::context::tls::enter_global::hdabbc8f603c57034
  38:     0x7f1997f5fa88 - rustc::ty::context::TyCtxt::create_and_enter::h9dc1c7ae44c9c683
  39:     0x7f1997eb12c7 - rustc_driver::driver::compile_input::h57481c89ea5db323
  40:     0x7f1997f2edd2 - rustc_driver::run_compiler_impl::h87832c1791e7dfc0
  41:     0x7f1997e9adb8 - syntax::with_globals::h159f70f74dc74b1a
  42:     0x7f1997ed2762 - std::sys_common::backtrace::__rust_begin_short_backtrace::h2eb0bf59b4be5e91
  43:     0x7f1997aea1ae - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  44:     0x7f1997ed3804 - std::panicking::try::hd2667e266321a08e
  45:     0x7f1997ee6cf2 - <F as alloc::boxed::FnBox<A>>::call_box::h95d70f3fd084ce4b
  46:     0x7f1997ac1d3d - std::sys::unix::thread::Thread::new::thread_start::h82597e85006aac29
                               at /home/mw/0-rust/src/liballoc/boxed.rs:793
                               at libstd/sys_common/thread.rs:24
                               at libstd/sys/unix/thread.rs:90
  47:     0x7f1991c2e6b9 - start_thread
  48:     0x7f199778441c - clone
  49:                0x0 - <unknown>
