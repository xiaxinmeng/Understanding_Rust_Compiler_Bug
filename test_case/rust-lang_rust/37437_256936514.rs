
stack backtrace:
   1:     0x7fd8d02ee03f - std::sys::backtrace::tracing::imp::write::h22f199c1dbb72ba2
   2:     0x7fd8d02fd59d - std::panicking::default_hook::{{closure}}::h9a389c462b6a22dd
   3:     0x7fd8d02fa9c6 - std::panicking::default_hook::h852b4223c1c00c59
   4:     0x7fd8d02fb0a8 - std::panicking::rust_panic_with_hook::hcd9d05f53fa0dafc
   5:     0x7fd8cc746657 - std::panicking::begin_panic::h2f463d37998ebeba
   6:     0x7fd8cc756578 - rustc_errors::Handler::bug::haca77c19c882b432
   7:     0x7fd8cd7b934a - rustc::session::opt_span_bug_fmt::{{closure}}::hfeb850fbe828b399
   8:     0x7fd8cd6f2ee5 - rustc::session::opt_span_bug_fmt::h46e45438a860a75e
   9:     0x7fd8cd6f2d22 - rustc::session::bug_fmt::hde22f071bf5a80ea
  10:     0x7fd8cee95910 - <collections::vec::Vec<T> as core::iter::traits::FromIterator<T>>::from_iter::hc5cbb62358ceb057
  11:     0x7fd8cef3b64f - rustc_trans::mir::rvalue::<impl rustc_trans::mir::MirContext<'bcx, 'tcx>>::trans_rvalue::h2a14de75e2fd4285
  12:     0x7fd8cef29969 - rustc_trans::mir::block::<impl rustc_trans::mir::MirContext<'bcx, 'tcx>>::trans_block::he67d3259f79e4177
  13:     0x7fd8cef27c58 - rustc_trans::mir::trans_mir::h2fb44ecb31cfdffa
  14:     0x7fd8ceec7b83 - rustc_trans::base::trans_closure::h941de14309416d66
  15:     0x7fd8cef47cd4 - rustc_trans::trans_item::TransItem::define::h2e690ccd1ee22f95
  16:     0x7fd8ceecb07a - rustc_trans::base::trans_crate::h9b06de31ed8799d1
  17:     0x7fd8d06827ed - rustc_driver::driver::phase_4_translate_to_llvm::hc3883ea2c4750179
  18:     0x7fd8d06ba8b5 - rustc_driver::driver::compile_input::{{closure}}::h9162a2fa292aeb3f
  19:     0x7fd8d06b1bb7 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h1928c4704cfe9c61
  20:     0x7fd8d0680224 - rustc_driver::driver::phase_3_run_analysis_passes::he578df6b8805151c
  21:     0x7fd8d0667bfb - rustc_driver::driver::compile_input::h5b63ccd49eeeb98b
  22:     0x7fd8d06931da - rustc_driver::run_compiler::h98c7274e7cb1d11d
  23:     0x7fd8d05cfe0b - std::panicking::try::do_call::h99ed0da044e497c3
  24:     0x7fd8d0305496 - __rust_maybe_catch_panic
  25:     0x7fd8d05ed951 - <F as alloc::boxed::FnBox<A>>::call_box::hbdd5a14cd8e33b97
  26:     0x7fd8d02f9420 - std::sys::thread::Thread::new::thread_start::h50b05608a499d2b2
  27:     0x7fd8c85e2453 - start_thread
  28:     0x7fd8cffc97de - __GI___clone
  29:                0x0 - <unknown>
