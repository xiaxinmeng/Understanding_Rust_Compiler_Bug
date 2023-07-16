
$ RUST_BACKTRACE=full rustc -Zunstable-options --unpretty=mir 1.rs

error: no method named `iter` found for type `Iterate<{integer}, [closure@1.rs:26:24: 26:31]>` in the current scope
  --> 1.rs:28:24
   |
28 |     println!("{:?}", a.iter().take(10).collect::<Vec<usize>>());
   |                        ^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `iter`, perhaps you need to implement it:
   = help: candidate #1: `core::slice::SliceExt`

error: internal compiler error: src/librustc/ty/sty.rs:1316: Ty::fn_sig() called on non-fn type: [type error]

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:420
stack backtrace:
   0:        0x114b3d433 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h34a7b169ac42b490
   1:        0x114b4a494 - std::panicking::default_hook::{{closure}}::hcc7f46907143dca1
   2:        0x114b4a02f - std::panicking::default_hook::h9cd165f254d86e53
   3:        0x114b4c9d7 - std::panicking::rust_panic_with_hook::h2bd2f0b446f0face
   4:        0x111906faa - std::panicking::begin_panic::hbe0b5616bfac2783
   5:        0x1119206ef - rustc_errors::Handler::bug::h10efb660bfa883cd
   6:        0x1109ac2e4 - rustc::session::opt_span_bug_fmt::{{closure}}::h0b26fd99d0a67776
   7:        0x1109ac1eb - rustc::session::opt_span_bug_fmt::ha596cf769754630c
   8:        0x1109abe5a - rustc::session::bug_fmt::hc8583ff8d4b58b73
   9:        0x11084d8d9 - rustc::cfg::construct::CFGBuilder::expr::h602bb82d21c17bd3
  10:        0x11084f581 - rustc::cfg::CFG::new::hea8e51a9b63234f8
  11:        0x10f77d19a - rustc_borrowck::borrowck::borrowck::h7060021ea44457b2
  12:        0x110a9fac2 - rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::force::h535570634d646f4e
  13:        0x10ff1c418 - rustc_mir::transform::optimized_mir::h1b9bf578672579b9
  14:        0x110a981e5 - rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::try_get::h7ed7add002451bae
  15:        0x110ab3a4a - rustc::ty::maps::TyCtxtAt::optimized_mir::h57fed91d26765566
  16:        0x110ab108e - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::optimized_mir::h975e521b12b7aa18
  17:        0x10ff2587e - rustc_mir::util::pretty::write_mir_pretty::h2e61090113b77b7f
  18:        0x10f6159a7 - rustc_driver::pretty::print_with_analysis::{{closure}}::h54efbed9d3460ecd
  19:        0x10f604794 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::hb8907c4bb5830ea5
  20:        0x10f5e15fc - rustc_driver::driver::phase_3_run_analysis_passes::h47d9ce9b36ad24f6
  21:        0x10f613c2b - rustc_driver::pretty::print_after_hir_lowering::ha3e95f8f5f8e75ec
  22:        0x10f61fd3b - <rustc_driver::RustcDefaultCalls as rustc_driver::CompilerCalls<'a>>::build_controller::{{closure}}::h7bff312ebb7d03c8
  23:        0x10f5cd843 - rustc_driver::driver::compile_input::hdef5f220808181b7
  24:        0x10f61a110 - rustc_driver::run_compiler::h20e73f67d559ff12
  25:        0x10f534ab8 - std::panicking::try::do_call::h61fa3e8a8d939c78
  26:        0x114b4f9ca - __rust_maybe_catch_panic
  27:        0x10f562daf - <F as alloc::boxed::FnBox<A>>::call_box::h4458443c22631385
  28:        0x114b494d5 - std::sys::imp::thread::Thread::new::thread_start::h5852519f9f48a1f2
  29:     0x7fffc9aa49ae - _pthread_body
  30:     0x7fffc9aa48fa - _pthread_start
