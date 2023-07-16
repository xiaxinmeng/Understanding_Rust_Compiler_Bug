
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `6`)', ../src/librustc_const_eval/check_match.rs:595
stack backtrace:
   1:        0x10cab66eb - std::sys::backtrace::tracing::imp::write::h3800f45f421043b8
   2:        0x10cac2eb5 - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0ef6c8db532f55dc
   3:        0x10cac29ef - std::panicking::default_hook::hf3839060ccbb8764
   4:        0x10ca88ba7 - std::panicking::rust_panic_with_hook::h5dd7da6bb3d06020
   5:        0x10cac3476 - std::panicking::begin_panic::h9bf160aee246b9f6
   6:        0x10ca8aad8 - std::panicking::begin_panic_fmt::haf08a9a70a097ee1
   7:        0x1091742a4 - rustc_const_eval::check_match::construct_witness::h07dc250246eb0869
   8:        0x10917ab17 - _<std..iter..Map<I, F> as std..iter..Iterator>::next::h051d30388b63ab57
   9:        0x1091584e4 - rustc_const_eval::check_match::is_useful::h016026cbd52060ce
  10:        0x109150f84 - rustc_const_eval::check_match::check_expr::hc6bacb6a9f0d0247
  11:        0x109152f8a - rustc_const_eval::check_match::check_fn::hec241997e15f2e67
  12:        0x109153660 - rustc_const_eval::check_match::check_crate::h6c69aca2c018b92e
  13:        0x1083e33d4 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h4bf34c9820399d99
  14:        0x1083dfeca - rustc::ty::context::TyCtxt::create_and_enter::hbfd876096454bbd0
  15:        0x1083a866e - rustc_driver::driver::compile_input::hda370d330171d8d7
  16:        0x1083949a4 - rustc_driver::run_compiler::ha942b7e1d33fe553
  17:        0x10839198f - std::panicking::try::call::h929be2db59c0ff06
  18:        0x10caca44b - __rust_try
  19:        0x10caca3e5 - __rust_maybe_catch_panic
  20:        0x1083924ab - _<F as std..boxed..FnBox<A>>::call_box::h6fc56a5ebed87bf3
  21:        0x10cac1a98 - std::sys::thread::Thread::new::thread_start::h9e5bde00f3b3e2e2
  22:     0x7fff86d3d99c - _pthread_body
  23:     0x7fff86d3d919 - _pthread_start


