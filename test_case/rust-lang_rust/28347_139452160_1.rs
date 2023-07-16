
test.rs:10:9: 10:24 error: internal compiler error: the 1th autoderef failed: _
test.rs:10         (&mut *closure)()
                   ^~~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', /home/boeckb/code/depot/group-tools/group-compilers/rust/src/src/libsyntax/diagnostic.rs:176

stack backtrace:
   1:     0x7f60c700f3f9 - sys::backtrace::tracing::imp::write::h8b75e77eba5376947Ds
   2:     0x7f60c7016236 - panicking::on_panic::h5275dab6a5c932f03nx
   3:     0x7f60c6fda43e - rt::unwind::begin_unwind_inner::ha828651abcb14bd2DQw
   4:     0x7f60c18a5ac7 - rt::unwind::begin_unwind::h14112450495354169214
   5:     0x7f60c18a5a86 - diagnostic::SpanHandler::span_bug::h25e95f8c16fb331c6NA
   6:     0x7f60c4ddaf93 - middle::infer::InferCtxt<'a, 'tcx>::adjust_expr_ty::h7088acddd2dd6615qgD
   7:     0x7f60c4d4c91b - middle::infer::InferCtxt<'a, 'tcx>::expr_ty_adjusted::h77cfa3384bb70a47dCD
   8:     0x7f60c4d2230d - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_expr::hba228794d7686b2e9Rq
   9:     0x7f60c4cd59de - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::consume_expr::h407b039a1e5fd44dQJq
  10:     0x7f60c4cd59de - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::consume_expr::h407b039a1e5fd44dQJq
  11:     0x7f60c4cd70aa - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_fn::hbb33fd59447aa4099Dq
  12:     0x7f60c5b10bcd - check::upvar::AdjustBorrowKind<'a, 'tcx>.Visitor<'v>::visit_fn::hc93176fa5128af14fbk
  13:     0x7f60c5b10840 - visit::walk_expr::h11868499041215646841
  14:     0x7f60c5b1056e - visit::walk_expr::h11868499041215646841
  15:     0x7f60c5b572c5 - check::check_bare_fn::h06986055badcbbf40yp
  16:     0x7f60c5b54f95 - check::check_item_body::h2301adf3dacd60e8k0p
  17:     0x7f60c5c0de82 - check_crate::h1b9403d493270ff35BE
  18:     0x7f60c75441dd - driver::phase_3_run_analysis_passes::closure.21868
  19:     0x7f60c75268ec - middle::ty::ctxt<'tcx>::create_and_enter::h4064699478710373221
  20:     0x7f60c7521e1e - driver::phase_3_run_analysis_passes::h6837423016930985994
  21:     0x7f60c75017dd - driver::compile_input::h9fd8d131527aff620ba
  22:     0x7f60c766a08b - run_compiler::hd075c291dca55a37qqc
  23:     0x7f60c76679a7 - boxed::F.FnBox<A>::call_box::h1679426590584622772
  24:     0x7f60c7667414 - rt::unwind::try::try_fn::h8745201937627540681
  25:     0x7f60c7015dd8 - __rust_try
  26:     0x7f60c7001fa2 - rt::unwind::try::inner_try::h47ab16ad5f7c2136wMw
  27:     0x7f60c76675a8 - boxed::F.FnBox<A>::call_box::h11492860850382891678
  28:     0x7f60c7015223 - sys::thread::Thread::new::thread_start::hc875ea6af05e7240vWv
  29:     0x7f60c10ea6c9 - start_thread
  30:     0x7f60c6c9499c - __clone
  31:                0x0 - <unknown>
