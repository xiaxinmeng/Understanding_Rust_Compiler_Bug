
error: internal compiler error: type_of with TyProjection
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', src/libsyntax/errors/mod.rs:230
stack backtrace:
   1:     0x7fd80e171460 - sys::backtrace::tracing::imp::write::h51a4398742da0623K0t
   2:     0x7fd80e177fa5 - panicking::log_panic::_<closure>::closure.41506
   3:     0x7fd80e177a67 - panicking::log_panic::hf3690f0da42f7f25Y7x
   4:     0x7fd80e141043 - sys_common::unwind::begin_unwind_inner::h53054637973a059djTs
   5:     0x7fd80a1d4657 - sys_common::unwind::begin_unwind::begin_unwind::h16847477020784951041
   6:     0x7fd80a1d4a14 - errors::Handler::bug::h623de609723e663bAZc
   7:     0x7fd80b5fd4ec - session::Session::bug::hf2ec81b68b650bda9es
   8:     0x7fd80cae9c7d - trans::type_of::in_memory_type_of::hb00ac4f71a5d2dc2bdQ
   9:     0x7fd80caed9cb - trans::base::alloc_ty::h4a46066a01d64342yhi
  10:     0x7fd80cc08f54 - trans::_match::mk_binding_alloca::h16584621283196570395
  11:     0x7fd80cb0986e - trans::base::init_local::h3cc56120bc50056al2h
  12:     0x7fd80cb23e12 - trans::controlflow::trans_block::h14e00c38eb84fe83VAw
  13:     0x7fd80cb204f7 - trans::base::trans_closure::hd70f9863156f5359bQi
  14:     0x7fd80cb2478c - trans::base::trans_fn::hf204ab03c9603db1XZi
  15:     0x7fd80cb28caf - trans::base::trans_item::hc86e02775fa6daedXnj
  16:     0x7fd80cb34c64 - trans::base::trans_crate::h1366c98d49e7fc59K7j
  17:     0x7fd80e6e50a1 - driver::phase_4_translate_to_llvm::h72f03170468523ffuTa
  18:     0x7fd80e6def36 - driver::phase_3_run_analysis_passes::_<closure>::closure.25890
  19:     0x7fd80e6bcc05 - middle::ty::context::ctxt<'tcx>::create_and_enter::create_and_enter::h12957577696868225661
  20:     0x7fd80e6b8521 - driver::phase_3_run_analysis_passes::h14675146087569771906
  21:     0x7fd80e68ceb9 - driver::compile_input::hcea038a856e2e417jca
  22:     0x7fd80e67f0cb - run_compiler::h6bc599602c13f1ce5wc
  23:     0x7fd80e67be86 - sys_common::unwind::try::try_fn::try_fn::h12460656089999964736
  24:     0x7fd80e16f0f8 - __rust_try
  25:     0x7fd80e1668eb - sys_common::unwind::try::inner_try::h193a98f8a6ded06bRPs
  26:     0x7fd80e67c1e0 - boxed::F.FnBox<A>::call_box::call_box::h4438575142861133636
  27:     0x7fd80e1764b3 - sys::thread::Thread::new::thread_start::he0b127204317aff8bax
  28:     0x7fd807a430a3 - start_thread
  29:     0x7fd80ddf504c - clone
  30:                0x0 - <unknown>
