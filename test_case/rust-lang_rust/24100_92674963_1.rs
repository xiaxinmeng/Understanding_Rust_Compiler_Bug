
$ RUST_BACKTRACE=1 rustc test.rs 
error: internal compiler error: translating unsupported cast: S (cast_other) -> S (cast_other)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/robn/code/rust/rust/src/libsyntax/diagnostic.rs:190

stack backtrace:
   1:     0x7f9ef6fdb0b8 - sys::backtrace::write::hc7bf68ee23aa2a47IJC
   2:     0x7f9ef7006460 - panicking::on_panic::hae54fd167ccd0c87sXI
   3:     0x7f9ef6f3e193 - rt::unwind::begin_unwind_inner::h04b19af4046d9402ACI
   4:     0x7f9ef430657d - rt::unwind::begin_unwind::h14358179414781977900
   5:     0x7f9ef4306cd2 - diagnostic::Handler::bug::h6b05bef8ecda0ec4ZsB
   6:     0x7f9ef4d1980b - session::Session::bug::h7677b9d1b16c63a2oYq
   7:     0x7f9ef668358a - trans::expr::trans_imm_cast::h3c51f5e7bab523a6nKj
   8:     0x7f9ef666b3b9 - trans::expr::trans_unadjusted::h50be0c1e615b9b53TXh
   9:     0x7f9ef6622944 - trans::expr::trans::h5fac271f51fcff39Blh
  10:     0x7f9ef6682347 - trans::expr::trans_addr_of::h6ef6c8160ed58d06fij
  11:     0x7f9ef666b67f - trans::expr::trans_unadjusted::h50be0c1e615b9b53TXh
  12:     0x7f9ef6620ff9 - trans::expr::trans_into::h03dc3dcf7bddc72dgfh
  13:     0x7f9ef669e4bd - trans::expr::trans_adt::h390328ce7ac619d86Yi
  14:     0x7f9ef666ee4e - trans::expr::trans_rvalue_dps_unadjusted::ha57d2ad1e4a3199amti
  15:     0x7f9ef666b1e0 - trans::expr::trans_unadjusted::h50be0c1e615b9b53TXh
  16:     0x7f9ef6622944 - trans::expr::trans::h5fac271f51fcff39Blh
  17:     0x7f9ef6719a60 - trans::_match::trans_match_inner::hc4bc6db15d8e6ce3gUw
  18:     0x7f9ef666ca8c - trans::expr::trans_rvalue_dps_unadjusted::ha57d2ad1e4a3199amti
  19:     0x7f9ef666b1e0 - trans::expr::trans_unadjusted::h50be0c1e615b9b53TXh
  20:     0x7f9ef6622944 - trans::expr::trans::h5fac271f51fcff39Blh
  21:     0x7f9ef6682347 - trans::expr::trans_addr_of::h6ef6c8160ed58d06fij
  22:     0x7f9ef666b67f - trans::expr::trans_unadjusted::h50be0c1e615b9b53TXh
  23:     0x7f9ef6622944 - trans::expr::trans::h5fac271f51fcff39Blh
  24:     0x7f9ef665a28f - trans::callee::trans_args::hea46cd98019ae23d9Rg
  25:     0x7f9ef6660d24 - trans::callee::trans_call_inner::h8036123679852670048
  26:     0x7f9ef666d583 - trans::expr::trans_rvalue_dps_unadjusted::ha57d2ad1e4a3199amti
  27:     0x7f9ef666b1e0 - trans::expr::trans_unadjusted::h50be0c1e615b9b53TXh
  28:     0x7f9ef6622944 - trans::expr::trans::h5fac271f51fcff39Blh
  29:     0x7f9ef665a28f - trans::callee::trans_args::hea46cd98019ae23d9Rg
  30:     0x7f9ef6660d24 - trans::callee::trans_call_inner::h8036123679852670048
  31:     0x7f9ef666d583 - trans::expr::trans_rvalue_dps_unadjusted::ha57d2ad1e4a3199amti
  32:     0x7f9ef6620fd8 - trans::expr::trans_into::h03dc3dcf7bddc72dgfh
  33:     0x7f9ef6620056 - trans::controlflow::trans_stmt_semi::hd76f97e4ff316b93L5d
  34:     0x7f9ef6621ca5 - trans::controlflow::trans_block::h2574d0aaf248b2a2H6d
  35:     0x7f9ef66f8d2f - trans::base::trans_closure::h4bbbebda4a6a2eaeQtt
  36:     0x7f9ef660ac76 - trans::base::trans_fn::hc59e1e3af67fd20dzEt
  37:     0x7f9ef660590f - trans::base::trans_item::h0c811983fce37f04P2t
  38:     0x7f9ef670126c - trans::base::trans_crate::h84a63b946dfd0dd2lZu
  39:     0x7f9ef766e84f - driver::phase_4_translate_to_llvm::h0f768fc8e75d0e95aOa
  40:     0x7f9ef764629b - driver::compile_input::h154756c108de519fQba
  41:     0x7f9ef76fd675 - run_compiler::hd45ff49ff4db7f9fV4b
  42:     0x7f9ef76faf8d - boxed::F.FnBox<A>::call_box::h998211653744645471
  43:     0x7f9ef76fa4c9 - rt::unwind::try::try_fn::h5486996895865591730
  44:     0x7f9ef708b748 - rust_try_inner
  45:     0x7f9ef708b735 - rust_try
  46:     0x7f9ef76fa79b - boxed::F.FnBox<A>::call_box::h4941640570998788665
  47:     0x7f9ef6ff1471 - sys::thread::create::thread_start::haf7c3f703308e235ExH
  48:     0x7f9ef0f480a3 - start_thread
  49:     0x7f9ef6ba804c - __clone
  50:                0x0 - <unknown>
