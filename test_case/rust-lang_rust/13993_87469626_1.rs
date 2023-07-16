
$ RUST_BACKTRACE=1 rustc gh13993.rs
gh13993.rs:5:20: 5:44 error: internal compiler error: impossible case reached: bad combination of types for cast
gh13993.rs:5   println!("{:?}", 1.0 as *const libc::FILE); // Can't cast float to foreign.
                                ^~~~~~~~~~~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:58 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
gh13993.rs:5:3: 5:46 note: expansion site
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:130

stack backtrace:
   1:     0x7f27bbf55809 - sys::backtrace::write::ha14749a03a5342e69AD
   2:     0x7f27bbf8011c - panicking::on_panic::hee4fd6319a5ce222fRJ
   3:     0x7f27bbea63e3 - rt::unwind::begin_unwind_inner::h7cec0e68cf4dd4e9WwJ
   4:     0x7f27b92987dd - rt::unwind::begin_unwind::h16368507805347149607
   5:     0x7f27b9298783 - diagnostic::SpanHandler::span_bug::hf68c3717c26bcf9eGcB
   6:     0x7f27b9fac525 - session::Session::impossible_case::h8b352797db0cae2bd6n
   7:     0x7f27bb633d51 - trans::consts::const_expr_unadjusted::h03a089cb940505b9SMn
   8:     0x7f27bb635b59 - trans::consts::const_expr::h3c3a9b1366d64db2lAn
   9:     0x7f27bb633ebd - trans::consts::const_expr_unadjusted::h03a089cb940505b9SMn
  10:     0x7f27bb635b59 - trans::consts::const_expr::h3c3a9b1366d64db2lAn
  11:     0x7f27bb63704c - iter::Iterator::fold::h7667499821084222699
  12:     0x7f27bb631fd7 - trans::consts::const_expr_unadjusted::h03a089cb940505b9SMn
  13:     0x7f27bb635b59 - trans::consts::const_expr::h3c3a9b1366d64db2lAn
  14:     0x7f27bb5c29e4 - trans::consts::get_const_expr_as_global::h63382b5d696b07d3uxn
  15:     0x7f27bb57b9dd - trans::expr::trans::hcf318f7829ca8538Mih
  16:     0x7f27bb672460 - trans::_match::trans_match_inner::he933589ceb1202f2lBw
  17:     0x7f27bb5c5f37 - trans::expr::trans_rvalue_dps_unadjusted::h964b5fa24a29fe8axqi
  18:     0x7f27bb5c4560 - trans::expr::trans_unadjusted::h10f55f6dfad913c24Uh
  19:     0x7f27bb57baef - trans::expr::trans::hcf318f7829ca8538Mih
  20:     0x7f27bb5db6c7 - trans::expr::trans_addr_of::he411e7f5950a250ayfj
  21:     0x7f27bb5c49ff - trans::expr::trans_unadjusted::h10f55f6dfad913c24Uh
  22:     0x7f27bb57baef - trans::expr::trans::hcf318f7829ca8538Mih
  23:     0x7f27bb5b367f - trans::callee::trans_args::hec695dff3d0fbc81eQg
  24:     0x7f27bb5ba123 - trans::callee::trans_call_inner::h12525817113047954229
  25:     0x7f27bb5c6a29 - trans::expr::trans_rvalue_dps_unadjusted::h964b5fa24a29fe8axqi
  26:     0x7f27bb5c4560 - trans::expr::trans_unadjusted::h10f55f6dfad913c24Uh
  27:     0x7f27bb57baef - trans::expr::trans::hcf318f7829ca8538Mih
  28:     0x7f27bb5b367f - trans::callee::trans_args::hec695dff3d0fbc81eQg
  29:     0x7f27bb5ba123 - trans::callee::trans_call_inner::h12525817113047954229
  30:     0x7f27bb5c6a29 - trans::expr::trans_rvalue_dps_unadjusted::h964b5fa24a29fe8axqi
  31:     0x7f27bb57a158 - trans::expr::trans_into::h2426cf3f6b952d64rch
  32:     0x7f27bb5791db - trans::controlflow::trans_stmt_semi::ha28b9ba4168939b0C5d
  33:     0x7f27bb57ae2a - trans::controlflow::trans_block::h61fb63fb3fbc91c6p6d
  34:     0x7f27bb6512ff - trans::base::trans_closure::h0f30c1dc4d13ade2kct
  35:     0x7f27bb563a86 - trans::base::trans_fn::h5b04b2d61da24ae13mt
  36:     0x7f27bb55e71f - trans::base::trans_item::h1d9b2463b34f325cVKt
  37:     0x7f27bb659804 - trans::base::trans_crate::h58934c6f3b222cb5rHu
  38:     0x7f27bc5f28ef - driver::phase_4_translate_to_llvm::h7a15f0e507d40c8diOa
  39:     0x7f27bc5ca41b - driver::compile_input::hd2f739199e3490b7Qba
  40:     0x7f27bc681b65 - run_compiler::hdf2665a0bb00f65fp2b
  41:     0x7f27bc67f61a - thunk::F.Invoke<A, R>::invoke::h5956351518329296155
  42:     0x7f27bc67e869 - rt::unwind::try::try_fn::h18011959801515578923
  43:     0x7f27bbffa388 - rust_try_inner
  44:     0x7f27bbffa375 - rust_try
  45:     0x7f27bc67ebc7 - thunk::F.Invoke<A, R>::invoke::h5384827922372053586
  46:     0x7f27bbf6b101 - sys::thread::create::thread_start::h5bfe55b0d48e24ddIqI
  47:     0x7f27b5d7d181 - start_thread
  48:     0x7f27bbb0c47c - __clone
  49:                0x0 - <unknown>
