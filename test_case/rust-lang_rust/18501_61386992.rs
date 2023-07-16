
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: did.krate != ast::LOCAL_CRATE', /home/bkoropoff/Source/rust/src/librustc/middle/ty.rs:4476

stack backtrace:
   1:     0x7f2749e6d600 - rt::backtrace::imp::write::h4bb43e154ca43b21Meq
   2:     0x7f2749e6f650 - failure::on_fail::hfc0fa620cf9857299zq
   3:     0x7f274e0afd00 - unwind::begin_unwind_inner::h67de795556f03740vTd
   4:     0x7f274e6e3bb0 - unwind::begin_unwind::h17429563519022666585
   5:     0x7f274eb10a90 - middle::ty::lookup_trait_def::h445f0408c992da73SsK
   6:     0x7f274e9dc8b0 - util::ppaux::ty..TraitRef.Repr::repr::ha3293d0710a14bb5lhK
   7:     0x7f274ea5db90 - middle::trans::common::fulfill_obligation::hce42b3bbe1630c53QJ8
   8:     0x7f274ea2f260 - middle::trans::meth::trans_static_method_callee::h6e647efcb386aec6kzm
   9:     0x7f274ea47180 - middle::trans::expr::trans_def::h404ea2cc1529cc808V5
  10:     0x7f274ea3f820 - middle::trans::expr::trans_unadjusted::hf278c1faf66daa53Es5
  11:     0x7f274ea0faf0 - middle::trans::expr::trans::h09d0376921b16455AL4
  12:     0x7f274ea39a00 - middle::trans::callee::trans_args::h98e66dc97db9b64dXi4
  13:     0x7f274ea17270 - middle::trans::callee::trans_call_inner::hba2fb74262554e0bMX3
  14:     0x7f274ea34480 - middle::trans::callee::trans_call::hfa1c38f86fbfcf6e6R3
  15:     0x7f274ea40ba0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h03c4a846155b8d5b555
  16:     0x7f274ea0e5c0 - middle::trans::expr::trans_into::h9ec3df557ec4a1d1GH4
  17:     0x7f274ea0db60 - middle::trans::controlflow::trans_stmt_semi::h2bf8c014bba1d5cbyX0
  18:     0x7f274ea0d180 - middle::trans::controlflow::trans_stmt::hfce27964ae879693lT0
  19:     0x7f274ea0e960 - middle::trans::controlflow::trans_block::h195c4a1576e5b6d0rY0
  20:     0x7f274ea9e060 - middle::trans::base::trans_closure::he382fee24a6c8113v5g
  21:     0x7f274ea038d0 - middle::trans::base::trans_fn::h95c9c34a4110b15fqhh
  22:     0x7f274ea01490 - middle::trans::base::trans_item::h080110be2df88ebdEAh
  23:     0x7f274e9fe2b0 - middle::trans::inline::instantiate_inline::h3dee3aa3860540219b0
  24:     0x7f274ea34730 - middle::trans::callee::trans_call::closure.122762
  25:     0x7f274ea17270 - middle::trans::callee::trans_call_inner::hba2fb74262554e0bMX3
  26:     0x7f274ea34480 - middle::trans::callee::trans_call::hfa1c38f86fbfcf6e6R3
  27:     0x7f274ea40ba0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h03c4a846155b8d5b555
  28:     0x7f274ea0e5c0 - middle::trans::expr::trans_into::h9ec3df557ec4a1d1GH4
  29:     0x7f274ea0db60 - middle::trans::controlflow::trans_stmt_semi::h2bf8c014bba1d5cbyX0
  30:     0x7f274ea0d180 - middle::trans::controlflow::trans_stmt::hfce27964ae879693lT0
  31:     0x7f274ea0e960 - middle::trans::controlflow::trans_block::h195c4a1576e5b6d0rY0
  32:     0x7f274ea9e060 - middle::trans::base::trans_closure::he382fee24a6c8113v5g
  33:     0x7f274ea038d0 - middle::trans::base::trans_fn::h95c9c34a4110b15fqhh
  34:     0x7f274ea01490 - middle::trans::base::trans_item::h080110be2df88ebdEAh
  35:     0x7f274eaa1c70 - middle::trans::base::trans_mod::ha859d0821e07a5419Fh
  36:     0x7f274eaa7a80 - middle::trans::base::trans_crate::h0e420231438a080dJyi
  37:     0x7f274ee70250 - driver::driver::phase_4_translate_to_llvm::h5eedb77a0534f86efOA
  38:     0x7f274ee68ef0 - driver::driver::compile_input::h3fb65e4233f7306c9kA
  39:     0x7f274eee6940 - driver::run::closure.145290
  40:     0x7f274e70f6b0 - task::TaskBuilder<S>::try_future::closure.103610
  41:     0x7f274e70f530 - task::TaskBuilder<S>::spawn_internal::closure.103557
  42:     0x7f274e3ede10 - task::NativeSpawner.Spawner::spawn::closure.8409
  43:     0x7f274e107d10 - rust_try_inner
  44:     0x7f274e107d00 - rust_try
  45:     0x7f274e0ada80 - task::Task::run::h42fb1dbb2643b426FNc
  46:     0x7f274e3edbe0 - task::NativeSpawner.Spawner::spawn::closure.8348
  47:     0x7f274e0aee70 - thread::thread_start::ha4aa6f5a9a81876aU8c
  48:     0x7f2749440e20 - start_thread
  49:     0x7f274dd7ab59 - clone
  50:                0x0 - <unknown>
