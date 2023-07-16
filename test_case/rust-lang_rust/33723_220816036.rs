
error: internal compiler error: ../src/librustc/infer/mod.rs:642: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<toxcore::dht::PackedNode as std::cmp::PartialOrd>)),depth=1),Unimplemented)]` resolving bounds after type-checking
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:545
stack backtrace:
   1:     0x7ffabbe72410 - std::sys::backtrace::tracing::imp::write::h9fb600083204ae7f
   2:     0x7ffabbe8004b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hca543c34f11229ac
   3:     0x7ffabbe7fbec - std::panicking::default_hook::hc2c969e7453d080c
   4:     0x7ffabbe455c8 - std::panicking::rust_panic_with_hook::hfe203e3083c2b544
   5:     0x7ffab98cb838 - std::panicking::begin_panic::hf2bac3a63b4cf82e
   6:     0x7ffab98cb6a5 - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::hf0fb5697f8f25f40
   7:     0x7ffab98cb46f - rustc::session::span_bug_fmt::h4591bcbadfc38558
   8:     0x7ffab996f30b - rustc::infer::InferCtxtBuilder::enter::_$u7b$$u7b$closure$u7d$$u7d$::ha6773c5e88930d4c
   9:     0x7ffab98e05cc - rustc_trans::common::fulfill_obligation::hb21eb8f16f9a76d1
  10:     0x7ffab9964791 - _<collector..MirNeighborCollector<'a, 'tcx> as rustc..mir..visit..Visitor<'tcx>>::visit_operand::hf0dd641ff1984693
  11:     0x7ffab9962269 - _<collector..MirNeighborCollector<'a, 'tcx> as rustc..mir..visit..Visitor<'tcx>>::visit_terminator_kind::h4eb0d7a8b3f87e9e
  12:     0x7ffab9960d64 - rustc::mir::visit::Visitor::visit_mir::h4b213039904e9146
  13:     0x7ffab995a1ea - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  14:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  15:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  16:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  17:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  18:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  19:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  20:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  21:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  22:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  23:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  24:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  25:     0x7ffab995af26 - rustc_trans::collector::collect_items_rec::h124468d64eb75cad
  26:     0x7ffab99216f3 - rustc_trans::base::collect_and_partition_translation_items::_$u7b$$u7b$closure$u7d$$u7d$::hb8ab0749c2e05a74
  27:     0x7ffab990b626 - rustc_trans::base::trans_crate::hbae75fb922e3217a
  28:     0x7ffabc3d8a63 - rustc_driver::driver::phase_4_translate_to_llvm::hf78914edeae2f0e8
  29:     0x7ffabc3d68d8 - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h9a2ae2a8248766a3
  30:     0x7ffabc3d30ed - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::he8e0792bce031dff
  31:     0x7ffabc3cc80b - rustc::ty::context::TyCtxt::create_and_enter::h9e0841510a9ae184
  32:     0x7ffabc39447c - rustc_driver::driver::compile_input::h0629572e6f316b31
  33:     0x7ffabc37f694 - rustc_driver::run_compiler::h8902aebf8b1849a8
  34:     0x7ffabc37c76e - std::panicking::try::call::hb9e578062982aefa
  35:     0x7ffabbe8e92b - __rust_try
  36:     0x7ffabbe8e8ce - __rust_maybe_catch_panic
  37:     0x7ffabc37d254 - _<F as std..boxed..FnBox<A>>::call_box::h27f542a39f1d61ef
  38:     0x7ffabbe7e154 - std::sys::thread::Thread::new::thread_start::h6f266e069bf4ec2b
  39:     0x7ffab3b45433 - start_thread
  40:     0x7ffabbadc28c - clone
  41:                0x0 - <unknown>
