
(gdb) bt
#0  0x00007f51e5ca44b7 in raise () from /usr/lib/libc.so.6
#1  0x00007f51e5ca588a in abort () from /usr/lib/libc.so.6
#2  0x00007f51e5c9d41d in __assert_fail_base () from /usr/lib/libc.so.6
#3  0x00007f51e5c9d4d2 in __assert_fail () from /usr/lib/libc.so.6
#4  0x00007f51e840e83e in llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&) ()
#5  0x00007f51e78e1f69 in llvm::IRBuilder<true, llvm::ConstantFolder, llvm::IRBuilderDefaultInserter<true> >::CreateCall(llvm::Value*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&) ()
#6  0x00007f51e839c1c6 in LLVMBuildCall ()
#7  0x00007f51e713f7af in trans::builder::Builder$LT$$u{27}a$C$$u{20}$u{27}tcx$GT$::call::h876586d31c9b3666aEr ()
#8  0x00007f51e7090d58 in trans::build::Call::had877430a1c1759aUoq ()
#9  0x00007f51e709ff40 in trans::base::invoke::he792c282be64deb9c0s ()
#10 0x00007f51e708aba2 in trans::callee::trans_lang_call::hba6b434045fe7e80ORg ()
#11 0x00007f51e7089325 in trans::controlflow::trans_fail::h577b588f6e7318127te ()
#12 0x00007f51e7107a30 in trans::base::fail_if_zero_or_overflows::h2d841423f525d47arTs ()
#13 0x00007f51e7106832 in trans::expr::trans_eager_binop::h773c177151cc0550tMj ()
#14 0x00007f51e70e3dd5 in trans::expr::trans_binary::hdaa99015117cbed4KVj ()
#15 0x00007f51e70ccc1a in trans::expr::trans_unadjusted::h4e139eeb46c210cdQli ()
#16 0x00007f51e70852b8 in trans::expr::trans_into::h225d9e6c2923fafdlAh ()
#17 0x00007f51e7183603 in trans::_match::mk_binding_alloca::h5359920857975042623 ()
#18 0x00007f51e70847ce in trans::base::init_local::hc1689d013dbfe470Zet ()
#19 0x00007f51e7085ad4 in trans::controlflow::trans_block::ha23567704a31fa16xae ()
#20 0x00007f51e715127b in trans::base::trans_closure::h53a2304d5cce6cecX3t ()
#21 0x00007f51e70721d1 in trans::base::trans_fn::he388c9e64e690a1f7eu ()
#22 0x00007f51e706dfca in trans::base::trans_item::hd8e315fbfc388007yDu ()
#23 0x00007f51e7152a69 in trans::base::trans_mod::habbfd992c0433f8fnJu ()
#24 0x00007f51e706d552 in trans::base::trans_item::hd8e315fbfc388007yDu ()
#25 0x00007f51e71572c9 in trans::base::trans_crate::hf665da3716c47588iAv ()
#26 0x00007f51e6f809be in driver::phase_4_translate_to_llvm::h661e1c54716425323Oa ()
#27 0x00007f51e6f5b460 in driver::compile_input::h1ed0a8b4db6185f4Cba ()
#28 0x00007f51e702bc64 in run_compiler::h7e8f027b670e6f197ac ()
#29 0x00007f51e7028fad in thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h10957786224773965967 ()
#30 0x00007f51e7027d6f in rt::unwind::try::try_fn::h11972162461772211631 ()
#31 0x00007f51e8cf2ce9 in rust_try_inner ()
#32 0x00007f51e8cf2cd6 in rust_try ()
#33 0x00007f51e70283bc in thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h17437111990391212063 ()
#34 0x00007f51e8ced366 in sys::thread::thread_start::h9f6730fbc95b528aUCB ()
#35 0x00007f51e6942374 in start_thread () from /usr/lib/libpthread.so.0
#36 0x00007f51e5d5927d in clone () from /usr/lib/libc.so.6
