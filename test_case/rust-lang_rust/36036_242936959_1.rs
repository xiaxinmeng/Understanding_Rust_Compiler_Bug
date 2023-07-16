
error: internal compiler error: ../src/librustc_trans/type_of.rs:155: failed to get layout for `Wrapper<Foobar>`: the type `()` has an unknown layout

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:644
stack backtrace:
   1:     0x7ffff756e833 - std::sys::backtrace::tracing::imp::write::h482d45d91246faa2
   2:     0x7ffff757f11d - std::panicking::default_hook::_{{closure}}::h89158f66286b674e
   3:     0x7ffff757d66d - std::panicking::default_hook::h9e30d428ee3b0c43
   4:     0x7ffff757dd68 - std::panicking::rust_panic_with_hook::h2224f33fb7bf2f4c
   5:     0x7fffefe18b97 - std::panicking::begin_panic::hd377747b5e679457
   6:     0x7fffefe2b208 - rustc_errors::Handler::bug::h5390c1898e6c2b8a
   7:     0x7ffff44f8f2a - rustc::session::opt_span_bug_fmt::_{{closure}}::h7c6beff1b8c2acab
   8:     0x7ffff4445055 - rustc::session::opt_span_bug_fmt::h8b8cb70ab73590aa
   9:     0x7ffff4444e92 - rustc::session::bug_fmt::ha0645f57a1d17c87
  10:     0x7ffff634eca2 - rustc_trans::type_of::sizing_type_of::hac6f4c390971a5ca
  11:     0x7ffff635fe94 - rustc_trans::abi::FnType::unadjusted::_{{closure}}::h09238684ac765f3d
  12:     0x7ffff62b146e - rustc_trans::abi::FnType::unadjusted::h881cc0f40154edcc
  13:     0x7ffff6308d79 - rustc_trans::declare::declare_fn::hb0a161c85d767c68
  14:     0x7ffff6347657 - rustc_trans::trans_item::TransItem::predefine::h43bc091dbd423801
  15:     0x7ffff62c58e1 - rustc_trans::base::trans_crate::hfc4e2ab878e1c72d
  16:     0x7ffff7937a92 - rustc_driver::driver::phase_4_translate_to_llvm::hc799bcf39cd85f66
  17:     0x7ffff7972ad0 - rustc_driver::driver::compile_input::_{{closure}}::h688e4abb1bbb42bf
  18:     0x7ffff79646a0 - rustc_driver::driver::phase_3_run_analysis_passes::_{{closure}}::h45e03cee16dcf298
  19:     0x7ffff78b5b7f - rustc::ty::context::TyCtxt::create_and_enter::h0021bc4caf3a2e06
  20:     0x7ffff7927094 - rustc_driver::driver::compile_input::hd9ecc57abd3cba85
  21:     0x7ffff794d18d - rustc_driver::run_compiler::h184264500271cc39
  22:     0x7ffff788ec43 - std::panicking::try::do_call::h17a7a17ad7240c5c
  23:     0x7ffff758d336 - __rust_maybe_catch_panic
  24:     0x7ffff78ac039 - _<F as alloc..boxed..FnBox<A>>::call_box::h95ef76c8d48c6f60
  25:     0x7ffff757bba0 - std::sys::thread::Thread::new::thread_start::he0bf102845911132
  26:     0x7ffff6cc7483 - start_thread
  27:     0x7ffff71c86dc - clone
  28:                0x0 - <unknown>
