
error: internal compiler error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:738: End-user description not implemented for field access on `TyGenerator(DefId(0/1:9 ~ foo[317d]::foo[0]::{{closure}}[0]), ClosureSubsts { substs: Slice([(), (), &mut u32]) }, GeneratorInterior { witness: (&mut u32, u32, &mut u32, (), &mut u32) })`

note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.24.0-nightly (687d3d15b 2018-01-02) running on x86_64-unknown-linux-gnu
note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:504:9
stack backtrace:
   0:     0x7fdc1506dbdb - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h1566ac160e99a5cb
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7fdc1507c79e - std::sys_common::backtrace::print::h7e125faafaec0730
                               at /checkout/src/libstd/sys_common/backtrace.rs:68
                               at /checkout/src/libstd/sys_common/backtrace.rs:57
   2:     0x7fdc15059660 - std::panicking::default_hook::{{closure}}::hd0e9c262265a893e
                               at /checkout/src/libstd/panicking.rs:381
   3:     0x7fdc15059123 - std::panicking::default_hook::h158ff347dc6943ee
                               at /checkout/src/libstd/panicking.rs:391
   4:     0x7fdc15059aab - std::panicking::rust_panic_with_hook::h2754d947e96f732e
                               at /checkout/src/libstd/panicking.rs:577
   5:     0x7fdc0fa30c97 - std::panicking::begin_panic::h31051ac2f3ce0711
   6:     0x7fdc0fa4b722 - rustc_errors::Handler::bug::h1d9869230db0d92f
   7:     0x7fdc10e1c8ef - <std::thread::local::LocalKey<T>>::with::h57fcb1ddb3467c90
   8:     0x7fdc11209aae - rustc::ty::context::tls::with_opt::h8ed9a4c5819c9e5e
   9:     0x7fdc10d4ce17 - rustc::session::opt_span_bug_fmt::hdc7c48ea259be04b
  10:     0x7fdc10d4cd26 - rustc::session::bug_fmt::ha284643e7da7e7e3
  11:     0x7fdc127677f2 - rustc_mir::borrow_check::error_reporting::<impl rustc_mir::borrow_check::MirBorrowckCtxt<'cx, 'gcx, 'tcx>>::describe_field_from_ty::ha17fe807f60896bb
  12:     0x7fdc12767362 - rustc_mir::borrow_check::error_reporting::<impl rustc_mir::borrow_check::MirBorrowckCtxt<'cx, 'gcx, 'tcx>>::describe_field::h9c58a4703cfb5c3f
  13:     0x7fdc12766d62 - rustc_mir::borrow_check::error_reporting::<impl rustc_mir::borrow_check::MirBorrowckCtxt<'cx, 'gcx, 'tcx>>::append_place_to_string::ha671b2657748c9a6
  14:     0x7fdc12766dd0 - rustc_mir::borrow_check::error_reporting::<impl rustc_mir::borrow_check::MirBorrowckCtxt<'cx, 'gcx, 'tcx>>::append_place_to_string::ha671b2657748c9a6
  15:     0x7fdc12765d3a - rustc_mir::borrow_check::error_reporting::<impl rustc_mir::borrow_check::MirBorrowckCtxt<'cx, 'gcx, 'tcx>>::report_borrowed_value_does_not_live_long_enough::he0d258764a1d83c5
  16:     0x7fdc1276ed81 - rustc_mir::borrow_check::MirBorrowckCtxt::access_place::hf9a2f24cbe56b497
  17:     0x7fdc1276d6c9 - <rustc_mir::borrow_check::MirBorrowckCtxt<'cx, 'gcx, 'tcx> as rustc_mir::dataflow::DataflowResultsConsumer<'cx, 'tcx>>::visit_terminator_entry::h3ade4cfed755a6d6
  18:     0x7fdc1276c3a9 - rustc_mir::borrow_check::do_mir_borrowck::hac81aa5201d63a65
  19:     0x7fdc1261fa6c - rustc::ty::context::tls::enter::h642ed5915575a2d0
  20:     0x7fdc126fac6d - rustc::infer::InferCtxtBuilder::enter::h93bf2ac5a0f0e2bf
  21:     0x7fdc1276b3cc - rustc_mir::borrow_check::mir_borrowck::h9ce0d2eba2dc1724
  22:     0x7fdc1108f86e - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::compute_result::hf2fd1ec56456a34d
  23:     0x7fdc110099fa - rustc::dep_graph::graph::DepGraph::with_task_impl::ha99d584b462f7361
  24:     0x7fdc10cfba90 - rustc_errors::Handler::track_diagnostics::hdb1fd5f71819c294
  25:     0x7fdc10bf9570 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::hda0dc39cd1541c00
  26:     0x7fdc1108f90f - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::force::he7ca8ee14cae8092
  27:     0x7fdc110901d3 - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::try_get::hcc4b25da4c8edf5b
  28:     0x7fdc10eb946e - rustc::ty::maps::TyCtxtAt::mir_borrowck::h54dd3692e5821103
  29:     0x7fdc10c1cd58 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck::h76c68fe1d3411521
  30:     0x7fdc12602292 - rustc_mir::transform::optimized_mir::hac5573c786cae4f7
  31:     0x7fdc1107f7e8 - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::compute_result::h3f083e6388044b38
  32:     0x7fdc11014679 - rustc::dep_graph::graph::DepGraph::with_task_impl::he84dacd9c4df3b3f
  33:     0x7fdc10cf1344 - rustc_errors::Handler::track_diagnostics::hab8a29d094d51986
  34:     0x7fdc10bcee70 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h52bfd31279321483
  35:     0x7fdc1107f87e - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::force::h37f8a5d3c0eaf4cb
  36:     0x7fdc1107ff99 - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::try_get::h5b26dec4d8915957
  37:     0x7fdc10eb8a9a - rustc::ty::maps::TyCtxtAt::optimized_mir::h1b7ef3c9f35af611
  38:     0x7fdc1120f0b8 - rustc::ty::sty::ClosureSubsts::field_tys::h5d29a42daaea5063
  39:     0x7fdc126e0a7d - rustc_mir::borrow_check::nll::type_check::type_check_internal::ha1f1c6f08948766c
  40:     0x7fdc12620925 - rustc::ty::context::tls::enter::h865e06d830bf98c4
  41:     0x7fdc126fabbb - rustc::infer::InferCtxtBuilder::enter::h75bde932c0ecfb7d
  42:     0x7fdc126ed0a6 - <rustc_mir::borrow_check::nll::type_check::TypeckMir as rustc_mir::transform::MirPass>::run_pass::he676413266873fb1
  43:     0x7fdc1260ae9e - rustc_mir::transform::mir_const::{{closure}}::h17f73615c9fd6a32
  44:     0x7fdc12601f3a - rustc_mir::transform::mir_const::h98f11695d2a0115d
  45:     0x7fdc1107cc68 - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::compute_result::h72e861f076ce6442
  46:     0x7fdc10ff98e9 - rustc::dep_graph::graph::DepGraph::with_task_impl::h65c8c0af599cf52e
  47:     0x7fdc10d01e74 - rustc_errors::Handler::track_diagnostics::hfd314462a46654a9
  48:     0x7fdc10be1540 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h928465c849d18cf5
  49:     0x7fdc1107ccfe - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::force::h290901aec7d14331
  50:     0x7fdc1107d439 - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::try_get::h5d75a326a70659b5
  51:     0x7fdc10eb88da - rustc::ty::maps::TyCtxtAt::mir_const::h33052c72cfb354cd
  52:     0x7fdc10c1cab0 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_const::h5c9469f59a685fd8
  53:     0x7fdc12602119 - rustc_mir::transform::mir_validated::h8a410f8efb9531e6
  54:     0x7fdc1107e228 - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::compute_result::hc629988bb87ae547
  55:     0x7fdc10ff98e9 - rustc::dep_graph::graph::DepGraph::with_task_impl::h65c8c0af599cf52e
  56:     0x7fdc10ce75e4 - rustc_errors::Handler::track_diagnostics::h72f70f4043bfe631
  57:     0x7fdc10bf98b0 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::hdbc5e8cd93946a0e
  58:     0x7fdc1107e2be - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::force::hf2dd59b30a8d9801
  59:     0x7fdc1107e9f9 - rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::try_get::h597822d239a5813e
  60:     0x7fdc10eb89ba - rustc::ty::maps::TyCtxtAt::mir_validated::hee096a7a83969778
  61:     0x7fdc10c1cae0 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_validated::h68d4825181cee416
  62:     0x7fdc14a260f7 - rustc_borrowck::borrowck::borrowck::he977a92684bb80ce
  63:     0x7fdc10fdecc0 - rustc::dep_graph::graph::DepGraph::with_task_impl::h041bb0d419f66155
  64:     0x7fdc10cde0de - rustc_errors::Handler::track_diagnostics::h47774c015bb76af4
  65:     0x7fdc10bd3820 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check::h5e93bc4e0263f657
  66:     0x7fdc1108debf - rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::force::h50ead84b561acf03
  67:     0x7fdc1108e6ce - rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::try_get::hba8b01b07a0109bc
  68:     0x7fdc10eb935c - rustc::ty::maps::TyCtxtAt::borrowck::h227dd66275277b95
  69:     0x7fdc10c1cd20 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::borrowck::h6a84f994c96fbbec
  70:     0x7fdc14a25d71 - rustc_borrowck::borrowck::check_crate::hfc72a7067f1ed908
  71:     0x7fdc15496501 - <std::thread::local::LocalKey<T>>::with::h314b95a705981850
  72:     0x7fdc15498e46 - <std::thread::local::LocalKey<T>>::with::h66de21815ebba7c9
  73:     0x7fdc154f1c37 - rustc::ty::context::TyCtxt::create_and_enter::h8bd3ff6ccc9dbd9f
  74:     0x7fdc15450e7a - rustc_driver::driver::compile_input::h5da4e74c985f17d8
  75:     0x7fdc154c61b0 - rustc_driver::run_compiler::h8c1ba0b94bb19857
  76:     0x7fdc1540bb71 - std::sys_common::backtrace::__rust_begin_short_backtrace::h8245a8c34ed63797
  77:     0x7fdc150a538e - __rust_maybe_catch_panic
                               at /checkout/src/libpanic_unwind/lib.rs:101
  78:     0x7fdc153eabb2 - <F as alloc::boxed::FnBox<A>>::call_box::h7388440a6f732841
  79:     0x7fdc1507f067 - std::sys_common::thread::start_thread::h0ff8375a6ab48652
                               at /checkout/src/liballoc/boxed.rs:827
                               at /checkout/src/libstd/sys_common/thread.rs:24
  80:     0x7fdc15083478 - std::sys::unix::thread::Thread::new::thread_start::h92ae5e7fea8a1d0f
                               at /checkout/src/libstd/sys/unix/thread.rs:90
  81:     0x7fdc0ef76519 - start_thread
  82:     0x7fdc14d4e37e - clone
  83:                0x0 - <unknown>
