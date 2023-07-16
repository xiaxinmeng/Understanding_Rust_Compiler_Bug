
src/lib.rs:4:13: 4:18 error: expected function, found `Foo`
src/lib.rs:4     let f = Foo();
                         ^~~~~
src/lib.rs:4:13: 4:16 error: internal compiler error: unexpected callee type Foo
src/lib.rs:4     let f = Foo();
                         ^~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7fc083810850 - sys::backtrace::write::hbd9ef428d64879c0K8t
   2:     0x7fc083833b90 - failure::on_fail::h94da1655857ca583CkB
   3:     0x7fc0837a01d0 - rt::unwind::begin_unwind_inner::h639a1a6cb0c8bc80DZA
   4:     0x7fc080ba5ab0 - rt::unwind::begin_unwind::h6684574056801298421
   5:     0x7fc080ba5a40 - diagnostic::SpanHandler::span_bug::hb9015874fb1e55c8eFE
   6:     0x7fc0816a1ad0 - session::Session::span_bug::h9306d474dc126165IMp
   7:     0x7fc082f61e50 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_expr::h709628170010510314
   8:     0x7fc082f61960 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_block::h1927624140920101121
   9:     0x7fc082f5aaa0 - check::upvar::AdjustBorrowKind<'a, 'tcx>::analyze_fn::h1721e1d330f7ab7aqPi
  10:     0x7fc082f8f8a0 - check::check_bare_fn::hdf0cdbfdf545dcaet3k
  11:     0x7fc082f87100 - check::check_item::hdb5c348d01fcd5bb6ll
  12:     0x7fc083056cb0 - check_crate::closure.32034
  13:     0x7fc083051790 - check_crate::hda609b05bfa80f85zNy
  14:     0x7fc083d9be90 - driver::phase_3_run_analysis_passes::h7398ce00da863c69NFa
  15:     0x7fc083d83270 - driver::compile_input::h4dbc958226844635Bba
  16:     0x7fc083e49480 - run_compiler::h6cae23f56147c839n9b
  17:     0x7fc083e47b10 - thunk::F.Invoke<A, R>::invoke::h7959888009317854762
  18:     0x7fc083e46a40 - rt::unwind::try::try_fn::h5318534964293814406
  19:     0x7fc08389ef10 - rust_try_inner
  20:     0x7fc08389ef00 - rust_try
  21:     0x7fc083e46cf0 - thunk::F.Invoke<A, R>::invoke::h10672339334941904280
  22:     0x7fc083820380 - sys::thread::thread_start::h23fd05fc04b5661fQ3w
  23:     0x7fc07da2e0c0 - start_thread
  24:     0x7fc083421fd9 - __clone
  25:                0x0 - <unknown>
