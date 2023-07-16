
thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:634
stack backtrace:
   1:        0x108878129 - std::sys::backtrace::tracing::imp::write::he2501374f73ec099
   2:        0x108886e70 - std::panicking::default_hook::_{{closure}}::hea6aa313ae4811e5
   3:        0x10888523f - std::panicking::default_hook::hbb5f2b9e0ee0a71b
   4:        0x108885946 - std::panicking::rust_panic_with_hook::h4b7cbd57bddc4336
   5:        0x1086aff8a - std::panicking::begin_panic::hd77fb6bc695c2c85
   6:        0x1086c0e17 - rustc_errors::Handler::bug::h6e4fc4cfb85c6a97
   7:        0x1054c73e6 - rustc::session::opt_span_bug_fmt::_{{closure}}::h1680ea7341a42104
   8:        0x105405b59 - rustc::session::opt_span_bug_fmt::hc08478f37a831072
   9:        0x10540590a - rustc::session::bug_fmt::he027e74bec7f01c2
  10:        0x10469ee1e - rustc_trans::type_of::unsized_info_ty::h92a01ad634be2477
  11:        0x10469f7e1 - rustc_trans::type_of::in_memory_type_of::hf10df1cac409b5a8
  12:        0x1046b23c2 - rustc_trans::abi::FnType::unadjusted::_{{closure}}::h690d3fe42713b1ff
  13:        0x1045af856 - rustc_trans::abi::FnType::unadjusted::h0358f343b16992a5
  14:        0x10462fccb - rustc_trans::declare::declare_fn::hd904e28439a7c612
  15:        0x104694a38 - rustc_trans::trans_item::TransItem::predefine::h2404554b21708aff
  16:        0x1045d06d5 - rustc_trans::base::trans_crate::h6f415e4caa44785a
  17:        0x10408a033 - rustc_driver::driver::phase_4_translate_to_llvm::ha88bac5acdbf871a
  18:        0x1040c9765 - rustc_driver::driver::compile_input::_{{closure}}::h1c506211f23b1f16
  19:        0x1040b1f89 - rustc_driver::driver::phase_3_run_analysis_passes::_{{closure}}::hb76b8eef12500013
  20:        0x10400f24c - rustc::ty::context::TyCtxt::create_and_enter::h121e851c393fc6c7
  21:        0x104074ba7 - rustc_driver::driver::compile_input::h080ab0005d7a6c8f
  22:        0x10409f7e9 - rustc_driver::run_compiler::hde5bcf2c18c8627d
  23:        0x103feb5d4 - std::panicking::try::do_call::hb7fee203b4478fbb
  24:        0x10888dc7a - __rust_maybe_catch_panic
  25:        0x104005247 - _<F as alloc..boxed..FnBox<A>>::call_box::h1b74254845a06a08
  26:        0x108884005 - std::sys::thread::Thread::new::thread_start::h403796632e47ee73
  27:     0x7fff8f389898 - _pthread_body
  28:     0x7fff8f389729 - _pthread_start
