
error: internal compiler error: ../src/librustc_trans/type_of.rs:166: Unexpected tail in unsized_info_ty: errors::AppError for ty=errors::AppError
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:575
stack backtrace:
   1:        0x10c3a23a8 - std::sys::backtrace::tracing::imp::write::h4c73fcd3363076f5
   2:        0x10c3ae7f5 - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0422dbb3077e6747
   3:        0x10c3ae32f - std::panicking::default_hook::haac48fa641db8fa2
   4:        0x10c372f86 - std::sys_common::unwind::begin_unwind_inner::h39d40f52add53ef7
   5:        0x10b970aca - std::sys_common::unwind::begin_unwind::hb6dfe67d47a39536
   6:        0x10b9708f9 - syntax::errors::Handler::bug::hec53df68774c03a0
   7:        0x10acf63dc - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::hda87a029f26efa73
   8:        0x10acf6241 - rustc::session::opt_span_bug_fmt::h9fde00dba42c5dbe
   9:        0x10ad0cad9 - rustc::session::bug_fmt::h3174fdeee89ebd23
  10:        0x108952524 - rustc_trans::type_of::unsized_info_ty::h31cd36d2b8d5affd
  11:        0x10882d0d2 - rustc_trans::type_of::in_memory_type_of::h3452b62d9f897a08
  12:        0x10881fbc1 - rustc_trans::abi::FnType::unadjusted::_$u7b$$u7b$closure$u7d$$u7d$::h43c6b7965bb3af53
  13:        0x10881d426 - rustc_trans::abi::FnType::unadjusted::h6c7ddebd02b4cfc6
  14:        0x10882cbbf - rustc_trans::type_of::in_memory_type_of::h3452b62d9f897a08
  15:        0x10888eaa8 - rustc_trans::callee::get_fn::haaf760d478d95498
  16:        0x108838b27 - rustc_trans::callee::Callee::def::he88e0427a7068fef
  17:        0x1088667b4 - rustc_trans::base::trans_item::hf167e469e36b8935
  18:        0x10887e416 - _<base..TransItemsWithinModVisitor<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::hfd0663a7c8d5f16c
  19:        0x10887c356 - rustc::hir::intravisit::walk_item::h511d7aa5180d79ae
  20:        0x10886ceae - rustc_trans::base::trans_crate::h62301dd3c79c73a7
  21:        0x107f0c73a - rustc_driver::driver::phase_4_translate_to_llvm::hd44e506f53bb81e3
  22:        0x107f0aeec - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h9811bece2a8f1da7
  23:        0x107f077dd - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::hc699330eb6f2bc1e
  24:        0x107f01392 - rustc::ty::context::TyCtxt::create_and_enter::h656b2d3a4956519e
  25:        0x107efdcdc - rustc_driver::driver::phase_3_run_analysis_passes::h83da042ec4b8ea10
  26:        0x107ed1571 - rustc_driver::driver::compile_input::h6491aaddd9e61258
  27:        0x107eb7d8f - rustc_driver::run_compiler::h80b2ba5e4d787c5f
  28:        0x107eb5122 - std::sys_common::unwind::try::try_fn::h34e27823ddd1d5e9
  29:        0x10c39fb3b - __rust_try
  30:        0x10c39fac3 - std::sys_common::unwind::inner_try::h9eebd8dc83f388a6
  31:        0x107eb59b9 - _<F as std..boxed..FnBox<A>>::call_box::h3d5d78986dfac5b2
  32:        0x10c3ad6c8 - std::sys::thread::Thread::new::thread_start::h471ad90789353b5b
  33:     0x7fff9350d99c - _pthread_body
  34:     0x7fff9350d919 - _pthread_start
