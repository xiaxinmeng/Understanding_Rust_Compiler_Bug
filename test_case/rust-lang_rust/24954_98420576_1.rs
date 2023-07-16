
test.rs:9:10: 3:15 error: re-assignment of immutable variable `x`
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'capacity overflow', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/option.rs:330

stack backtrace:
   1:     0x7f343bf6dee9 - sys::backtrace::write::h9d474f8e9af32ff4eas
   2:     0x7f343bf75a29 - panicking::on_panic::h780bc016e25d560eHOw
   3:     0x7f343bf36a52 - rt::unwind::begin_unwind_inner::he10b5af763481bfaRtw
   4:     0x7f343bf37817 - rt::unwind::begin_unwind_fmt::h9bb9f2d65e37b92bXsw
   5:     0x7f343bf75606 - rust_begin_unwind
   6:     0x7f343bfc1dd4 - panicking::panic_fmt::h11c0222b0ca0e381hJy
   7:     0x7f34392c59cf - codemap::CodeMap::span_to_lines::h669ff35ac8bd1b88tLA
   8:     0x7f343930cc27 - diagnostic::emit::h65da8787fd9d4fc9spC
   9:     0x7f343930a9e4 - diagnostic::EmitterWriter.Emitter::emit::he3bafb0e40df4f46amC
  10:     0x7f34392bcba1 - diagnostic::SpanHandler::span_err::h5614c50f1c18ca09gSB
  11:     0x7f3439f2519d - session::Session::span_err::h3d8ac9553125fc59WGq
  12:     0x7f343a81195e - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>.euv..Delegate<'tcx>::mutate::h43caac2e19e577aeyla
  13:     0x7f343a821c56 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::mutate_expr::h7171415938733347731
  14:     0x7f343a81ef08 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_expr::h2675070928678527945
  15:     0x7f343a821a23 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::consume_expr::h17812453369581463546
  16:     0x7f343a81eece - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_expr::h2675070928678527945
  17:     0x7f343a821a23 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::consume_expr::h17812453369581463546
  18:     0x7f343a813ede - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_fn::h21981989491015306
  19:     0x7f343a83bc0b - borrowck::borrowck_fn::h480089f89c2a71e5NOe
  20:     0x7f343a83b20f - borrowck::BorrowckCtxt<'a, 'tcx>.Visitor<'v>::visit_fn::h7c2e2fb9b1bcd595bGe
  21:     0x7f343a83e436 - visit::walk_item::h11293387340257430338
  22:     0x7f343a83dd39 - borrowck::check_crate::h6ae0d2f7513822c4QIe
  23:     0x7f343c4bc2e7 - driver::phase_3_run_analysis_passes::h876ca40ef22f652btGa
  24:     0x7f343c49bfec - driver::compile_input::h19dbb81f9d079a10Qba
  25:     0x7f343c554dc1 - run_compiler::h6fdcc5ae4819da2c65b
  26:     0x7f343c552612 - boxed::F.FnBox<A>::call_box::h15867338134671801927
  27:     0x7f343c551bd9 - rt::unwind::try::try_fn::h10571205104630549427
  28:     0x7f343bfe92f8 - rust_try_inner
  29:     0x7f343bfe92e5 - rust_try
  30:     0x7f343c551e74 - boxed::F.FnBox<A>::call_box::h11752498478748955554
  31:     0x7f343bf747c1 - sys::thread::Thread::new::thread_start::h9f92b31902592327dAv
  32:     0x7f3436005181 - start_thread
  33:     0x7f343bbba47c - __clone
  34:                0x0 - <unknown>

rustc 1.1.0-nightly (c42c1e7a6 2015-05-02) (built 2015-05-02)
binary: rustc
commit-hash: c42c1e7a678a27bb63c24fb1eca2866af4e3ab7a
commit-date: 2015-05-02
build-date: 2015-05-02
host: x86_64-unknown-linux-gnu
release: 1.1.0-nightly
