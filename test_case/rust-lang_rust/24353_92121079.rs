
error: internal compiler error: trans_local_var: no datum for local/arg 26 found
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:190

stack backtrace:
   1:     0x7f69c383c3c8 - sys::backtrace::write::h179e141147f9ee14IJC
   2:     0x7f69c3867770 - panicking::on_panic::h6aa569d33389f0cfsXI
   3:     0x7f69c379f4a3 - rt::unwind::begin_unwind_inner::h082f07869f4b9731ACI
   4:     0x7f69c0b6504d - rt::unwind::begin_unwind::h13428428822164950735
   5:     0x7f69c0b657a2 - diagnostic::Handler::bug::h7dec5f2940917255ZsB
   6:     0x7f69c157882b - session::Session::bug::hc743ab3e9fbf0b0bnYq
   7:     0x7f69c2eefd78 - trans::expr::trans_local_var::hd7325e6f6b345fc2NMi
   8:     0x7f69c2ee6591 - trans::expr::trans_def::h2fd28f24ac3b6462rli
   9:     0x7f69c2ed2ce5 - trans::expr::trans_unadjusted::hf93051714e2bb83eTXh
  10:     0x7f69c2e88889 - trans::expr::trans_into::h6740c9e159fab14cgfh
  11:     0x7f69c2e898d2 - trans::controlflow::trans_block::hca8dabb22968b279H6d
  12:     0x7f69c2f605bf - trans::base::trans_closure::h2b9d4a88090cb2c4Qtt
  13:     0x7f69c2e72506 - trans::base::trans_fn::h13eecdaa7f8f7dbfzEt
  14:     0x7f69c2e6d19f - trans::base::trans_item::hcd09515ced7a23baP2t
  15:     0x7f69c2f68afc - trans::base::trans_crate::hdffc9e3841dc9fb5lZu
  16:     0x7f69c3ec7bbf - driver::phase_4_translate_to_llvm::hfb4ae79c3a76d704aOa
  17:     0x7f69c3e9f60b - driver::compile_input::h7c62c2870ff901f6Qba
  18:     0x7f69c3f56a25 - run_compiler::hc815b49ce638c28eV4b
  19:     0x7f69c3f5433d - boxed::F.FnBox<A>::call_box::h4290333406520961126
  20:     0x7f69c3f53879 - rt::unwind::try::try_fn::h13999576404798567721
  21:     0x7f69c38e3c68 - rust_try_inner
  22:     0x7f69c38e3c55 - rust_try
  23:     0x7f69c3f53b4b - boxed::F.FnBox<A>::call_box::h14489442628055152240
  24:     0x7f69c3852781 - sys::thread::create::thread_start::h02b7da32b4d7a4aaExH
  25:     0x7f69bd643373 - start_thread
  26:     0x7f69c341227c - __clone
  27:                0x0 - <unknown>
