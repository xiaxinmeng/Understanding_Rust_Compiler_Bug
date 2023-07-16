
thread 'rustc' panicked at 'explicit panic', ../src/librustc_errors/lib.rs:417
stack backtrace:
   1:        0x1111b2df8 - std::sys::backtrace::tracing::imp::write::h22f199c1dbb72ba2
   2:        0x1111c067f - std::panicking::default_hook::{{closure}}::h9a389c462b6a22dd
   3:        0x1111bdb2d - std::panicking::default_hook::h852b4223c1c00c59
   4:        0x1111be1c6 - std::panicking::rust_panic_with_hook::hcd9d05f53fa0dafc
   5:        0x111091454 - std::panicking::begin_panic::hc03e2830c2c89a5f
   6:        0x1110a04b7 - <rustc_errors::DiagnosticBuilder<'a> as core::ops::Drop>::drop::hb41536d1b29c0a0d
   7:        0x111e33a60 - drop::ha8549908cdb27fbb
   8:        0x111e333d7 - drop::h9905e368ae460fe8
   9:        0x111e73ea4 - docopt_macros::expand::h6d29a5b0a5787255
  10:        0x10d4450b5 - <F as syntax::ext::base::TTMacroExpander>::expand::hd81ff0fe7c85f753
  11:        0x110de3b82 - syntax::ext::expand::MacroExpander::expand::h737d7f1635448d5d
  12:        0x110de1b75 - syntax::ext::expand::MacroExpander::expand_crate::hd89d06b7a6333976
  13:        0x110decc3d - syntax::ext::expand::expand_crate::h80012253cfa7f1f5
  14:        0x10d085e3b - rustc_driver::driver::phase_2_configure_and_expand::{{closure}}::hc795707fb140075e
  15:        0x10d03e285 - rustc_driver::driver::phase_2_configure_and_expand::h04526112c643cb41
  16:        0x10d038c62 - rustc_driver::driver::compile_input::h5b63ccd49eeeb98b
  17:        0x10d061169 - rustc_driver::run_compiler::h98c7274e7cb1d11d
  18:        0x10cf9ffd8 - std::panicking::try::do_call::h99ed0da044e497c3
  19:        0x1111c0c3a - __rust_maybe_catch_panic
  20:        0x10cfbefef - <F as alloc::boxed::FnBox<A>>::call_box::hbdd5a14cd8e33b97
  21:        0x1111bcde4 - std::sys::thread::Thread::new::thread_start::h50b05608a499d2b2
  22:     0x7fff9752baba - _pthread_body
  23:     0x7fff9752ba06 - _pthread_start
