
error: internal compiler error: ../src/librustc_trans/mir/operand.rs:82: impossible case reached
thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:634
stack backtrace:
   1:     0x7fa594550cdd - std::sys::backtrace::tracing::imp::write::h29f5fdb9fc0a7395
   2:     0x7fa594561a71 - std::panicking::default_hook::_{{closure}}::h17d8437f66223ab1
   3:     0x7fa59455fed8 - std::panicking::default_hook::hbbe7fa36a995aca0
   4:     0x7fa5945606b8 - std::panicking::rust_panic_with_hook::h105c3d42fcd2fb5e
   5:     0x7fa58c2f6aa7 - std::panicking::begin_panic::hccc513334ab977d2
   6:     0x7fa58c30764a - rustc_errors::Handler::bug::h9fe35eb8aa73a45f
   7:     0x7fa590dd2334 - rustc::session::opt_span_bug_fmt::_{{closure}}::h62b0957667555cfe
   8:     0x7fa590d0e345 - rustc::session::opt_span_bug_fmt::hb71219f119a31511
   9:     0x7fa590d0e182 - rustc::session::bug_fmt::h8de2935acd9a57e4
  10:     0x7fa593444acf - _<collections..vec..Vec<T> as core..iter..traits..FromIterator<T>>::from_iter::
h2cbb281f2d52df9f
  11:     0x7fa593541109 - rustc_trans::mir::rvalue::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::tra
ns_rvalue::hdb93ac455c9e5a8c
  12:     0x7fa59353088c - rustc_trans::mir::block::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::tran
s_block::hd8dbf4a4fd69198b
  13:     0x7fa59352e618 - rustc_trans::mir::trans_mir::h2a26a93b40b40e45
  14:     0x7fa593489894 - rustc_trans::base::trans_closure::he79a96cff38dfe5c
  15:     0x7fa593550603 - rustc_trans::trans_item::TransItem::define::h1e5a74077917a073
  16:     0x7fa59348f566 - rustc_trans::base::trans_crate::h9baf3a0389546061
  17:     0x7fa594aecaad - rustc_driver::driver::phase_4_translate_to_llvm::h54e99578fb1bb696
  18:     0x7fa594b2bf96 - rustc_driver::driver::compile_input::_{{closure}}::h7236bd0d96fe8ce9
  19:     0x7fa594b13b4e - rustc_driver::driver::phase_3_run_analysis_passes::_{{closure}}::h7f2cec505064b
4bf
  20:     0x7fa594a71e9e - rustc::ty::context::TyCtxt::create_and_enter::he5dca3f2a7a0810a
  21:     0x7fa594ad64f8 - rustc_driver::driver::compile_input::hb4cc34cf85dc1edf
  22:     0x7fa594b00602 - rustc_driver::run_compiler::h50f95674bd902ab5
  23:     0x7fa594a47ddd - std::panicking::try::call::h31fc30b58c55d6c3
  24:     0x7fa59456ff16 - __rust_maybe_catch_panic
  25:     0x7fa594a60ac0 - _<F as alloc..boxed..FnBox<A>>::call_box::h24f3eb0b42327962
  26:     0x7fa59455e202 - std::sys::thread::Thread::new::thread_start::h8f3bd45211e9f5ea
  27:     0x7fa58b9eb6f9 - start_thread
  28:     0x7fa5941a0b5c - clone
  29:                0x0 - <unknown>
