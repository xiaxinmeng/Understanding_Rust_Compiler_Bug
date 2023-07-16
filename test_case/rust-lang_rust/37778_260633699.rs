
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore/option.rs:323
stack backtrace:
   1:        0x10837199a - std::sys::imp::backtrace::tracing::imp::write::he3d1bfbdbf113480
   2:        0x10838175f - std::panicking::default_hook::{{closure}}::h575f1b40d2e88f07
   3:        0x10837e825 - std::panicking::default_hook::h3d5dccce8125d0cf
   4:        0x10837ef56 - std::panicking::rust_panic_with_hook::h00b81bb3dcbd51f2
   5:        0x10837edf4 - std::panicking::begin_panic::ha6a0d553db9869ff
   6:        0x10837ed12 - std::panicking::begin_panic_fmt::h24d113aee3ee4081
   7:        0x10837ec77 - rust_begin_unwind
   8:        0x1083c1b00 - core::panicking::panic_fmt::he441b2ea2036b98a
   9:        0x1083c1a04 - core::panicking::panic::haf296e94ad32f436
  10:        0x1045d155a - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_expr::hafa0684345698985
  11:        0x10458ed4a - rustc::hir::intravisit::walk_expr::hd5abdde71da475ae
  12:        0x1045d1121 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_expr::hafa0684345698985
  13:        0x1045cd7e0 - rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_expr::h567b39e30ac6e41f
  14:        0x104618d74 - rustc_typeck::check::check_const_with_type::h7863ca898613a42a
  15:        0x1046131c7 - rustc_typeck::check::check_item_type::haf6b4f88a74e9c2e
  16:        0x10460dc1f - <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h8d60f5904f34ee84
  17:        0x10460f4de - rustc_typeck::check::check_item_types::had23c1548c5185a3
  18:        0x104676c17 - rustc_typeck::check_crate::h6539dedcf4cc6a82
  19:        0x103c8cccc - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::hb062c53ab7b33bac
  20:        0x103c5a616 - rustc_driver::driver::phase_3_run_analysis_passes::h016090d638ff1d7f
  21:        0x103c4d5b5 - rustc_driver::driver::compile_input::h21840cdf516c3ab1
  22:        0x103c77d39 - rustc_driver::run_compiler::h81a62653df4e7b03
  23:        0x103bb6d18 - std::panicking::try::do_call::h54deb93462da3a7f
  24:        0x108381d1a - __rust_maybe_catch_panic
  25:        0x103bd5d0f - <F as alloc::boxed::FnBox<A>>::call_box::h2181ecf694b72547
  26:        0x10837ddc4 - std::sys::imp::thread::Thread::new::thread_start::h990fb082eb5abe34
  27:     0x7fff986d299c - _pthread_body
  28:     0x7fff986d2919 - _pthread_start
