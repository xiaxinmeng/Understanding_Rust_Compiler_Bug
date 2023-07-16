
RUST_BACKTRACE=1 ./x86_64-apple-darwin/stage1/bin/rustc test.rs
error: main function not found
test.rs:4:10: 4:15 error: internal compiler error: no enclosing scope found for scope: CodeExtent(5/Misc(7))
test.rs:4     Some(ref p) => p.to_str().unwrap().to_owned(),
                   ^~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', src/libsyntax/errors/mod.rs:530
stack backtrace:
   1:        0x10fc2b056 - sys::backtrace::tracing::imp::write::h7006513b9258115esgu
   2:        0x10fc33525 - panicking::default_handler::_<closure>::closure.42883
   3:        0x10fc3309e - panicking::default_handler::h90c66ac48d04fb1fuAy
   4:        0x10fbfaf06 - sys_common::unwind::begin_unwind_inner::ha6bc2fa13f22ffb8Kft
   5:        0x10bdb477b - sys_common::unwind::begin_unwind::begin_unwind::h9247066333824177465
   6:        0x10bdb471b - errors::Handler::span_bug::span_bug::h188106568327982822
   7:        0x10be4f247 - check::dropck::check_safety_of_destructor_if_necessary::hcf54e33cf8188fd0pva
   8:        0x10be63137 - util::walk_pat::walk_pat_::walk_pat_::h12187628368062413369
   9:        0x10be631f3 - util::walk_pat::walk_pat_::walk_pat_::h12187628368062413369
  10:        0x10be63bd4 - intravisit::walk_expr::walk_expr::h11870600260843383917
  11:        0x10be61723 - check::regionck::visit_expr::h945a515ebaf89badaVc
  12:        0x10be4adde - check::regionck::regionck_expr::ha59b22dbbdd1dbd3Lnc
  13:        0x10be4ac3f - check::check_const_with_ty::hc02b7c5e54b330a4a4s
  14:        0x10bdd9e46 - check::check_const::hfae7ac30634e8ac4U2s
  15:        0x10bdc3888 - check::check_item_type::h0585f0b20ed09aceGVo
  16:        0x10bdc2dee - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::hd38bc8fa857b8495azo
  17:        0x10bdbf341 - check::check_item_types::h66fcfdc7605d1f46bBo
  18:        0x10bdb6dbe - check_crate::h65bb169362185961zBC
  19:        0x10bb1f778 - driver::phase_3_run_analysis_passes::_<closure>::closure.27725
  20:        0x10bb09796 - middle::ty::context::ctxt<'tcx>::create_and_enter::create_and_enter::h6007028584866556321
  21:        0x10bb053a4 - driver::phase_3_run_analysis_passes::h9652791452303199723
  22:        0x10bad2036 - driver::compile_input::haf1bda63295274a7Bca
  23:        0x10bac3aef - run_compiler::hfa3630c99d8476308Gc
  24:        0x10bac0a19 - sys_common::unwind::try::try_fn::try_fn::h10559814892422177770
  25:        0x10fc28ae8 - __rust_try
  26:        0x10fc20998 - sys_common::unwind::inner_try::hebf4874c09c80c3e0ct
  27:        0x10bac1271 - boxed::F.FnBox<A>::call_box::call_box::h9489525047418053242
  28:        0x10fc324bf - sys::thread::Thread::new::thread_start::h3f597cef5cd16b4bVOx
  29:     0x7fff90582c12 - _pthread_body
  30:     0x7fff90582b8f - _pthread_start
