
bar.rs:4:5: 4:9 warning: path statement with no effect, #[warn(path_statements)] on by default
bar.rs:4     BAZ;
             ^~~~
bar.rs:4:5: 4:8 error: internal compiler error: constant expression should not reach expr::trans_def
bar.rs:4     BAZ;
             ^~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/diagnostic.rs:176

stack backtrace:
   1:     0x7fdcb574a48e - sys::backtrace::write::h9d7a90b82951696dZws
   2:     0x7fdcb5752795 - panicking::on_panic::h84e509cf0bec31052lx
   3:     0x7fdcb57139ce - rt::unwind::begin_unwind_inner::h2e32ca1bf051967dF1w
   4:     0x7fdcb2b427cc - rt::unwind::begin_unwind::h16335744487247261469
   5:     0x7fdcb2b4276b - diagnostic::SpanHandler::span_bug::h1300244878ddb7aeb9A
   6:     0x7fdcb3601ca8 - session::Session::span_bug::h93d3986bee8b5267mat
   7:     0x7fdcb45dc7f3 - trans::expr::trans_def::h0cc06b0217f1e27c1TB
   8:     0x7fdcb45d3f7c - trans::expr::trans_unadjusted::h231a5476fd4b3a6eAwB
   9:     0x7fdcb45ad720 - trans::expr::trans_into::h202510f14f809442WSA
  10:     0x7fdcb45ad106 - trans::controlflow::trans_stmt_semi::h14851b10f841f7db9Hv
  11:     0x7fdcb453e0ca - trans::controlflow::trans_block::h7b997b267421ef305Iv
  12:     0x7fdcb453d160 - trans::base::trans_closure::h6c1a660000c76f88ixi
  13:     0x7fdcb453ea38 - trans::base::trans_fn::h040bb295d71555c25Gi
  14:     0x7fdcb4541ae8 - trans::base::trans_item::h07c7a6fa646583f7g6i
  15:     0x7fdcb454ea6c - trans::base::trans_crate::h34f1d7ba225f9cf7dUj
  16:     0x7fdcb5cbec04 - driver::phase_4_translate_to_llvm::hf081ebedb088ee245Oa
  17:     0x7fdcb5cb9245 - driver::phase_3_run_analysis_passes::closure.16406
  18:     0x7fdcb5cb349b - middle::ty::ctxt<'tcx>::create_and_enter::h5957637365875790786
  19:     0x7fdcb5cae471 - driver::phase_3_run_analysis_passes::h8973828644736070193
  20:     0x7fdcb5c92630 - driver::compile_input::h90abd5c73807fe0dTba
  21:     0x7fdcb5d76853 - run_compiler::he6382510493c3ff3A7b
  22:     0x7fdcb5d742ce - boxed::F.FnBox<A>::call_box::h2004289560739142696
  23:     0x7fdcb5d73bf9 - rt::unwind::try::try_fn::h2041046763977595402
  24:     0x7fdcb575225f - __rust_try_inner
  25:     0x7fdcb575229a - __rust_try
  26:     0x7fdcb573d907 - rt::unwind::try::inner_try::h9bf0939b00d7b9e6yXw
  27:     0x7fdcb5d73e18 - boxed::F.FnBox<A>::call_box::h13582013758951967072
  28:     0x7fdcb5751361 - sys::thread::Thread::new::thread_start::hd79e684f6fbf95bfD6v
  29:     0x7fdcaf35b6a9 - start_thread
  30:     0x7fdcb53a7eec - clone
  31:                0x0 - <unknown>
