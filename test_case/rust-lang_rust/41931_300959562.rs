
  * frame #0: 0x00007ffff734ca10 libc.so.6`__GI_raise + 272
    frame #1: 0x00007ffff734e13a libc.so.6`__GI_abort + 378
    frame #2: 0x00007ffff7345607 libc.so.6`__assert_fail_base + 295
    frame #3: 0x00007ffff73456b2 libc.so.6`__GI___assert_fail + 66
    frame #4: 0x00007ffff16be026 librustc_llvm-3aedc27cea629d08.so`llvm::cast_retty<llvm::Constant, llvm::Value*>::ret_type llvm::dyn_cast<llvm::Constant, llvm::Value>(llvm::Value*) + 54
    frame #5: 0x00007ffff265a4b1 librustc_llvm-3aedc27cea629d08.so`LLVMBuildInBoundsGEP + 49
    frame #6: 0x00007ffff6e7768f librustc_trans-8524c596fdbfd5aa.so`rustc_trans::builder::Builder::gepi::h048106edbbd4e5a5 + 319
    frame #7: 0x00007ffff6ea5522 librustc_trans-8524c596fdbfd5aa.so`rustc_trans::meth::VirtualIndex::get_fn::h7d313e8438bc4a04 + 34
    frame #8: 0x00007ffff6eb3c9b librustc_trans-8524c596fdbfd5aa.so`rustc_trans::mir::block::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$GT$::trans_argument::he2c25770e7173669 + 187
    frame #9: 0x00007ffff6eafeef librustc_trans-8524c596fdbfd5aa.so`rustc_trans::mir::block::_$LT$impl$u20$rustc_trans..mir..MirContext$LT$$u27$a$C$$u20$$u27$tcx$GT$$GT$::trans_block::hdf0af23de53006bf + 16319
    frame #10: 0x00007ffff6ea9d76 librustc_trans-8524c596fdbfd5aa.so`rustc_trans::mir::trans_mir::h3dfa63dadef8b96d + 17206
    frame #11: 0x00007ffff6ecd693 librustc_trans-8524c596fdbfd5aa.so`rustc_trans::trans_item::TransItem::define::h081f6bc1ba18c3e8 + 2291
    frame #12: 0x00007ffff6e6795a librustc_trans-8524c596fdbfd5aa.so`rustc_trans::base::trans_crate::hace7d592cac048a9 + 5258
    frame #13: 0x00007ffff7b42e4d librustc_driver-0f4024af7b82b2df.so`rustc_driver::driver::phase_4_translate_to_llvm::h904e3fc572f42e2d + 2413
    frame #14: 0x00007ffff7b020d9 librustc_driver-0f4024af7b82b2df.so`rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::hc476f85d4e1ed8fa + 1689
    frame #15: 0x00007ffff7b36261 librustc_driver-0f4024af7b82b2df.so`rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::ha83538a03579857c + 18209
    frame #16: 0x00007ffff7a726a4 librustc_driver-0f4024af7b82b2df.so`rustc::ty::context::TyCtxt::create_and_enter::hd9482ac47b3dbb7c + 4692
    frame #17: 0x00007ffff7b2a95f librustc_driver-0f4024af7b82b2df.so`rustc_driver::driver::phase_3_run_analysis_passes::hfea44285662197cc + 10271
    frame #18: 0x00007ffff7af80e5 librustc_driver-0f4024af7b82b2df.so`rustc_driver::driver::compile_input::hc74994fea2a7b3d8 + 9253
    frame #19: 0x00007ffff7b56fed librustc_driver-0f4024af7b82b2df.so`rustc_driver::run_compiler::hccc057d367b00a4b + 3213
    frame #20: 0x00007ffff7a2083c librustc_driver-0f4024af7b82b2df.so`std::panicking::try::do_call::hbc2a3c31903904db + 860
    frame #21: libstd-f4594d3e53dcb114.so`panic_unwind::__rust_maybe_catch_panic at lib.rs:98
    frame #22: 0x00007ffff7a668b2 librustc_driver-0f4024af7b82b2df.so`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h28281d8d856d67df + 146
    frame #23: libstd-f4594d3e53dcb114.so`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] alloc::boxed::{{impl}}::call_once<(),()> at boxed.rs:650
    frame #24: libstd-f4594d3e53dcb114.so`std::sys::imp::thread::{{impl}}::new::thread_start [inlined] std::sys_common::thread::start_thread at thread.rs:21
    frame #25: libstd-f4594d3e53dcb114.so`std::sys::imp::thread::{{impl}}::new::thread_start at thread.rs:84
    frame #26: 0x00007fffef6ec2e7 libpthread.so.0`start_thread + 215
    frame #27: 0x00007ffff740554f libc.so.6`__GI___clone + 63
