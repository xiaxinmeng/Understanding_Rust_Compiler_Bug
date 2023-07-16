
   Compiling igd v0.4.2 (file:///home/andreas/git/rust-igd)
     Running `rustc src/lib.rs --crate-name igd --crate-type lib -g --out-dir /home/andreas/git/rust-igd/target/debug --emit=dep-info,link -L dependency=/home/andreas/git/rust-igd/target/debug -L dependency=/home/andreas/git/rust-igd/target/debug/deps --extern rand=/home/andreas/git/rust-igd/target/debug/deps/librand-c724acb3942597d1.rlib --extern regex=/home/andreas/git/rust-igd/target/debug/deps/libregex-08fd4c31cabb9147.rlib --extern hyper=/home/andreas/git/rust-igd/target/debug/deps/libhyper-308072fd099a24f5.rlib --extern xmltree=/home/andreas/git/rust-igd/target/debug/deps/libxmltree-a22dd0794f673ad1.rlib --extern xml=/home/andreas/git/rust-igd/target/debug/deps/libxml-926334bf7dc36a15.rlib`
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: declare::get_defined_value(ccx, &symbol).is_none()', ../src/librustc_trans/monomorphize.rs:94
stack backtrace:
   1:     0x7f5dd959f300 - std::sys::backtrace::tracing::imp::write::h63529ecac330b5bc
   2:     0x7f5dd95ac9db - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0e99487804ecdaa3
   3:     0x7f5dd95ac577 - std::panicking::default_hook::h1f0a2403a716c4a6
   4:     0x7f5dd95702ff - std::sys_common::unwind::begin_unwind_inner::heae36551373e36bd
   5:     0x7f5dd87d2fef - std::sys_common::unwind::begin_unwind::h422885d31fe250dc
   6:     0x7f5dd887ab8d - rustc_trans::callee::get_fn::h16d5e12c52696e18
   7:     0x7f5dd8825920 - rustc_trans::callee::Callee::def::h6bfa8c0d18edb04c
   8:     0x7f5dd88eb597 - rustc_trans::glue::trans_struct_drop::h342318524db364ab
   9:     0x7f5dd88e8357 - rustc_trans::glue::get_drop_glue_core::ha9c0e47754271034
  10:     0x7f5dd8893be5 - rustc_trans::glue::drop_ty_core::hf833c0eb521a39ed
  11:     0x7f5dd88ec78a - rustc_trans::base::iter_structural_ty::hcad56d80bbb4b302
  12:     0x7f5dd88e8457 - rustc_trans::glue::get_drop_glue_core::ha9c0e47754271034
  13:     0x7f5dd8893be5 - rustc_trans::glue::drop_ty_core::hf833c0eb521a39ed
  14:     0x7f5dd88ec78a - rustc_trans::base::iter_structural_ty::hcad56d80bbb4b302
  15:     0x7f5dd88e8457 - rustc_trans::glue::get_drop_glue_core::ha9c0e47754271034
  16:     0x7f5dd8893be5 - rustc_trans::glue::drop_ty_core::hf833c0eb521a39ed
  17:     0x7f5dd88ec78a - rustc_trans::base::iter_structural_ty::hcad56d80bbb4b302
  18:     0x7f5dd88e8457 - rustc_trans::glue::get_drop_glue_core::ha9c0e47754271034
  19:     0x7f5dd8893be5 - rustc_trans::glue::drop_ty_core::hf833c0eb521a39ed
  20:     0x7f5dd88924b0 - _<cleanup..DropValue<'tcx> as cleanup..Cleanup<'tcx>>::trans::h012c5d4dc25d481b
  21:     0x7f5dd8891a57 - _<common..FunctionContext<'blk, 'tcx> as cleanup..CleanupHelperMethods<'blk, 'tcx>>::trans_cleanups_to_exit_scope::h25235d0ebda5e4a3
  22:     0x7f5dd889315b - _<common..FunctionContext<'blk, 'tcx> as cleanup..CleanupHelperMethods<'blk, 'tcx>>::get_or_create_landing_pad::h1b3312c7060bb49f
  23:     0x7f5dd882f133 - _<common..FunctionContext<'blk, 'tcx> as cleanup..CleanupMethods<'blk, 'tcx>>::get_landing_pad::h70c61d90463d8794
  24:     0x7f5dd882ed20 - rustc_trans::base::invoke::h4b34892eb6150166
  25:     0x7f5dd8824b4a - rustc_trans::callee::Callee::call::h721e5fc65601b1ff
  26:     0x7f5dd88d83ee - rustc_trans::expr::trans_rvalue_dps_unadjusted::hd53c161e5ecd9e4f
  27:     0x7f5dd884e234 - rustc_trans::expr::trans_into::h0886054c82efd10c
  28:     0x7f5dd891229d - rustc_trans::_match::mk_binding_alloca::hd19203cafc37a598
  29:     0x7f5dd883032d - rustc_trans::base::init_local::h617967a698cd0329
  30:     0x7f5dd8845794 - rustc_trans::controlflow::trans_block::h78eebe402333d4c3
  31:     0x7f5dd8843e37 - rustc_trans::base::trans_closure::hce4cad2baba38a75
  32:     0x7f5dd8846380 - rustc_trans::base::trans_fn::he5c9f3096cef336c
  33:     0x7f5dd8850901 - rustc_trans::base::trans_item::h2e00a2a37f0adf00
  34:     0x7f5dd886a3fb - _<base..TransItemsWithinModVisitor<'a, 'tcx> as rustc_front..intravisit..Visitor<'v>>::visit_item::h42989a4ea480287a
  35:     0x7f5dd88683f7 - rustc_front::intravisit::walk_item::h7a37cbc827b77b8a
  36:     0x7f5dd8857c0f - rustc_trans::base::trans_crate::h5d7d0ab38cd208e7
  37:     0x7f5dd9af7078 - rustc_driver::driver::phase_4_translate_to_llvm::hc60a4fd01d59eaae
  38:     0x7f5dd9af5905 - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h96aedfca5f48d7d6
  39:     0x7f5dd9af24e8 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h1d5b5af335ae8126
  40:     0x7f5dd9aec49b - rustc::ty::context::TyCtxt::create_and_enter::h2656c85b5a638887
  41:     0x7f5dd9ae8f19 - rustc_driver::driver::phase_3_run_analysis_passes::h432eb92c0e12df32
  42:     0x7f5dd9abb807 - rustc_driver::driver::compile_input::hb8958b270f32a505
  43:     0x7f5dd9aa9324 - rustc_driver::run_compiler::h146dabfc96bf3a5a
  44:     0x7f5dd9aa68a1 - std::sys_common::unwind::try::try_fn::h847694ac14db1c2b
  45:     0x7f5dd959c95b - __rust_try
  46:     0x7f5dd959c8ed - std::sys_common::unwind::inner_try::h790ed072af125c7c
  47:     0x7f5dd9aa70ea - _<F as alloc..boxed..FnBox<A>>::call_box::h64418c3ccdc3072e
  48:     0x7f5dd95aaba4 - std::sys::thread::Thread::new::thread_start::hdaf9c8e3f2f1c46e
  49:     0x7f5dd17000a3 - start_thread
  50:     0x7f5dd91ff87c - clone
  51:                0x0 - <unknown>

error: Could not compile `igd`.

Caused by:
  Process didn't exit successfully: `rustc src/lib.rs --crate-name igd --crate-type lib -g --out-dir /home/andreas/git/rust-igd/target/debug --emit=dep-info,link -L dependency=/home/andreas/git/rust-igd/target/debug -L dependency=/home/andreas/git/rust-igd/target/debug/deps --extern rand=/home/andreas/git/rust-igd/target/debug/deps/librand-c724acb3942597d1.rlib --extern regex=/home/andreas/git/rust-igd/target/debug/deps/libregex-08fd4c31cabb9147.rlib --extern hyper=/home/andreas/git/rust-igd/target/debug/deps/libhyper-308072fd099a24f5.rlib --extern xmltree=/home/andreas/git/rust-igd/target/debug/deps/libxmltree-a22dd0794f673ad1.rlib --extern xml=/home/andreas/git/rust-igd/target/debug/deps/libxml-926334bf7dc36a15.rlib` (exit code: 101)
