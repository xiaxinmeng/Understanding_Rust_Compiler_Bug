
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: declare::get_defined_value(ccx, &symbol).is_none()', ../src/librustc_trans/monomorphize.rs
:94
stack backtrace:
   1:     0x7f46f295b260 - std::sys::backtrace::tracing::imp::write::hc787ac7b725fc252
   2:     0x7f46f296893b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hff309ab1d83ffd90
   3:     0x7f46f29684d7 - std::panicking::default_hook::h08ad3bb09872855b
   4:     0x7f46f292c24f - std::sys_common::unwind::begin_unwind_inner::hfa66df7b3e69707e
   5:     0x7f46f14bffdf - std::sys_common::unwind::begin_unwind::h422885d31fe250dc
   6:     0x7f46f1567db5 - rustc_trans::callee::get_fn::h80509230b00b2d45
   7:     0x7f46f1512b10 - rustc_trans::callee::Callee::def::ha59f7b3a44ed318c
   8:     0x7f46f15d8807 - rustc_trans::glue::trans_struct_drop::hdf610ff8cbc85372
   9:     0x7f46f15d551f - rustc_trans::glue::get_drop_glue_core::he8ee956e8079a382
  10:     0x7f46f1580e55 - rustc_trans::glue::drop_ty_core::hc06aa6b2c77ee88e
  11:     0x7f46f15d99fa - rustc_trans::base::iter_structural_ty::hb996c6f6439d346e
  12:     0x7f46f15d5621 - rustc_trans::glue::get_drop_glue_core::he8ee956e8079a382
  13:     0x7f46f1580e55 - rustc_trans::glue::drop_ty_core::hc06aa6b2c77ee88e
  14:     0x7f46f15d99fa - rustc_trans::base::iter_structural_ty::hb996c6f6439d346e
  15:     0x7f46f15d5621 - rustc_trans::glue::get_drop_glue_core::he8ee956e8079a382
  16:     0x7f46f1580e55 - rustc_trans::glue::drop_ty_core::hc06aa6b2c77ee88e
  17:     0x7f46f15d99fa - rustc_trans::base::iter_structural_ty::hb996c6f6439d346e
  18:     0x7f46f15d5621 - rustc_trans::glue::get_drop_glue_core::he8ee956e8079a382
  19:     0x7f46f1580e55 - rustc_trans::glue::drop_ty_core::hc06aa6b2c77ee88e
  20:     0x7f46f157f720 - _<cleanup..DropValue<'tcx> as cleanup..Cleanup<'tcx>>::trans::hfe743496a676c6bd
  21:     0x7f46f157ecc7 - _<common..FunctionContext<'blk, 'tcx> as cleanup..CleanupHelperMethods<'blk, 'tcx>>::trans_cleanups_to_exit_s
cope::h70ffd6084bb51bb8
  22:     0x7f46f15803cb - _<common..FunctionContext<'blk, 'tcx> as cleanup..CleanupHelperMethods<'blk, 'tcx>>::get_or_create_landing_pa
d::hab4f305faaf87d7c
  23:     0x7f46f151c323 - _<common..FunctionContext<'blk, 'tcx> as cleanup..CleanupMethods<'blk, 'tcx>>::get_landing_pad::he3727711541cdf63
  24:     0x7f46f151bf10 - rustc_trans::base::invoke::ha7baf7e086d93914
  25:     0x7f46f1511d3a - rustc_trans::callee::Callee::call::h40441c69ded50749
  26:     0x7f46f15d887d - rustc_trans::glue::trans_struct_drop::hdf610ff8cbc85372
  27:     0x7f46f15d8314 - rustc_trans::glue::trans_struct_drop_flag::h0e7b4e9edbe6cbbc
  28:     0x7f46f15d5888 - rustc_trans::glue::get_drop_glue_core::he8ee956e8079a382
  29:     0x7f46f1580e55 - rustc_trans::glue::drop_ty_core::hc06aa6b2c77ee88e
  30:     0x7f46f157f720 - _<cleanup..DropValue<'tcx> as cleanup..Cleanup<'tcx>>::trans::hfe743496a676c6bd
  31:     0x7f46f1532e5a - _<common..FunctionContext<'blk, 'tcx> as cleanup..CleanupMethods<'blk, 'tcx>>::pop_and_trans_custom_cleanup_s
cope::hb68d67ac515ef149
  32:     0x7f46f15f5c71 - rustc_trans::_match::trans_match_inner::hbdd6056757522568
  33:     0x7f46f15c3c15 - rustc_trans::expr::trans_rvalue_dps_unadjusted::h2767d79242f1a8da
  34:     0x7f46f153b434 - rustc_trans::expr::trans_into::hccfa250f984a4de0
  35:     0x7f46f1532cad - rustc_trans::controlflow::trans_block::h85160ce006cd8aa8
  36:     0x7f46f1531027 - rustc_trans::base::trans_closure::h2b7da2bc6c23f7b2
  37:     0x7f46f1533570 - rustc_trans::base::trans_fn::h9d408db44d2e9f24
  38:     0x7f46f1566c71 - rustc_trans::callee::get_fn::h80509230b00b2d45
  39:     0x7f46f1512b10 - rustc_trans::callee::Callee::def::ha59f7b3a44ed318c
  40:     0x7f46f156172f - rustc_trans::callee::Callee::method_call::h91c54111eae6060e
  41:     0x7f46f15c4b4f - rustc_trans::expr::trans_rvalue_dps_unadjusted::h2767d79242f1a8da
  42:     0x7f46f153b434 - rustc_trans::expr::trans_into::hccfa250f984a4de0
  43:     0x7f46f15ff50d - rustc_trans::_match::mk_binding_alloca::h48ba7e46da50b940
  44:     0x7f46f151d51d - rustc_trans::base::init_local::h5b725441afc1979c
  45:     0x7f46f1532984 - rustc_trans::controlflow::trans_block::h85160ce006cd8aa8
  46:     0x7f46f1531027 - rustc_trans::base::trans_closure::h2b7da2bc6c23f7b2
  47:     0x7f46f1533570 - rustc_trans::base::trans_fn::h9d408db44d2e9f24
  48:     0x7f46f153db21 - rustc_trans::base::trans_item::h6ce564dd6c26e0a3
  49:     0x7f46f155763b - _<base..TransItemsWithinModVisitor<'a, 'tcx> as rustc_front..intravisit..Visitor<'v>>::visit_item::h17575fb31
3345f09
  50:     0x7f46f1555637 - rustc_front::intravisit::walk_item::h3f8f84eead983919
  51:     0x7f46f1544ddf - rustc_trans::base::trans_crate::h1aff7a8470cdce2a
  52:     0x7f46f2ebb3e8 - rustc_driver::driver::phase_4_translate_to_llvm::h7b8775f4874fc83b
  53:     0x7f46f2eb9c75 - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h31c654547881d0b4
  54:     0x7f46f2eb6858 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h25b1c02cc580d282
  55:     0x7f46f2eb080b - rustc::ty::context::TyCtxt::create_and_enter::h567cfc6893966048
  56:     0x7f46f2ead289 - rustc_driver::driver::phase_3_run_analysis_passes::h925d8f426a8a3ebf
  57:     0x7f46f2e7fb47 - rustc_driver::driver::compile_input::h4de0250909a780b6
  58:     0x7f46f2e6d5d4 - rustc_driver::run_compiler::hb9ea120b81672dbd
  59:     0x7f46f2e6ab31 - std::sys_common::unwind::try::try_fn::h10e40fd87dec6cc0
  60:     0x7f46f29588bb - __rust_try
  61:     0x7f46f295884d - std::sys_common::unwind::inner_try::h010cc5014fd950a2
  62:     0x7f46f2e6b37a - _<F as std..boxed..FnBox<A>>::call_box::hbe6d6868b645ad75
  63:     0x7f46f2966b04 - std::sys::thread::Thread::new::thread_start::h003cd23941cb846b
  64:     0x7f46ea8353f3 - start_thread
                        at /builddir/glibc-2.23/nptl/pthread_create.c:333
  65:     0x7f46f25bdbac - clone
                        at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
  66:                0x0 - <unknown>

error: Could not compile `incipient-webserver-rust`.
