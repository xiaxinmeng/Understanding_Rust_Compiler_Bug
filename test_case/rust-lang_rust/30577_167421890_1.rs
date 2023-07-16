
#0  0x00007ffff70ce5f8 in raise () from /usr/lib/libc.so.6
#1  0x00007ffff70cfa7a in abort () from /usr/lib/libc.so.6
#2  0x00007ffff70c7417 in __assert_fail_base () from /usr/lib/libc.so.6
#3  0x00007ffff70c74c2 in __assert_fail () from /usr/lib/libc.so.6
#4  0x00007ffff1af3670 in llvm::cast_retty<llvm::GlobalVariable, llvm::Value*>::ret_type llvm::cast<llvm::GlobalVariable, llvm::Value>(llvm::Value*) [clone .part.315] () from /usr/lib/librustc_llvm-17a8ccbd.so
#5  0x00007ffff1af7759 in LLVMGetInitializer () from /usr/lib/librustc_llvm-17a8ccbd.so
#6  0x00007ffff5f216bc in trans::consts::const_deref::h4c3a8b09e8457c33vrt () from /usr/lib/librustc_trans-17a8ccbd.so
#7  0x00007ffff5f20204 in trans::consts::const_expr::ha3c4de4b4bca8e8bFFt () from /usr/lib/librustc_trans-17a8ccbd.so
#8  0x00007ffff5f24500 in trans::consts::const_expr_unadjusted::h44f98122ca3bd1a5BXt () from /usr/lib/librustc_trans-17a8ccbd.so
#9  0x00007ffff5f1ff93 in trans::consts::const_expr::ha3c4de4b4bca8e8bFFt () from /usr/lib/librustc_trans-17a8ccbd.so
#10 0x00007ffff5eca923 in trans::base::trans_item::h9c19e9e4d1e730e5Xnj () from /usr/lib/librustc_trans-17a8ccbd.so
#11 0x00007ffff5ed6c55 in trans::base::trans_crate::h0270125f4db782cbK7j () from /usr/lib/librustc_trans-17a8ccbd.so
#12 0x00007ffff7a631d2 in driver::phase_4_translate_to_llvm::hbcd26352c79b2357uTa () from /usr/lib/librustc_driver-17a8ccbd.so
#13 0x00007ffff7a5d077 in driver::phase_3_run_analysis_passes::_$LT$closure$GT$::closure.26225 () from /usr/lib/librustc_driver-17a8ccbd.so
#14 0x00007ffff7a3ada6 in middle::ty::context::ctxt$LT$$u27$tcx$GT$::create_and_enter::create_and_enter::h6909965444811752025 () from /usr/lib/librustc_driver-17a8ccbd.so
#15 0x00007ffff7a366c2 in driver::phase_3_run_analysis_passes::h1635207307752584168 () from /usr/lib/librustc_driver-17a8ccbd.so
#16 0x00007ffff7a0b20a in driver::compile_input::h2a35f55d38a51b9ejca () from /usr/lib/librustc_driver-17a8ccbd.so
#17 0x00007ffff79fd41c in run_compiler::ha112fd42f27cc2125wc () from /usr/lib/librustc_driver-17a8ccbd.so
#18 0x00007ffff79fa1d7 in sys_common::unwind::try::try_fn::try_fn::h16052710914380154563 () from /usr/lib/librustc_driver-17a8ccbd.so
#19 0x00007ffff74f5b09 in __rust_try () from /usr/lib/libstd-17a8ccbd.so
#20 0x00007ffff74ed30c in sys_common::unwind::try::inner_try::hf02767e744fc6763dZs () from /usr/lib/libstd-17a8ccbd.so
#21 0x00007ffff79fa531 in boxed::F.FnBox$LT$A$GT$::call_box::call_box::h9288839064392637460 () from /usr/lib/librustc_driver-17a8ccbd.so
#22 0x00007ffff74fcec4 in sys::thread::Thread::new::thread_start::h1da3018ef918f37crjx () from /usr/lib/libstd-17a8ccbd.so
#23 0x00007ffff02944a4 in start_thread () from /usr/lib/libpthread.so.0
#24 0x00007ffff718413d in clone () from /usr/lib/libc.so.6
