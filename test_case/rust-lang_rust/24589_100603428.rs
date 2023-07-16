
(gdb) bt
#0  0x00007ffff7165165 in *__GI_raise (sig=<optimized out>) at ../nptl/sysdeps/unix/sysv/linux/raise.c:64
#1  0x00007ffff71683e0 in *__GI_abort () at abort.c:92
#2  0x00007ffff715e311 in *__GI___assert_fail (
    assertion=0x7ffff3fc64b0 "getOperand(0)->getType() == cast<PointerType>(getOperand(1)->getType())->getElementType() && \"Ptr must be a pointer to Val type!\"", file=<optimized out>, line=1083, function=0x7ffff3fc9fe0 "void llvm::StoreInst::AssertOK()")
    at assert.c:81
#3  0x00007ffff347c541 in llvm::StoreInst::AssertOK() () from ../.prefix/lib/librustc_llvm-4e7c5e5c.so
#4  0x00007ffff2340fea in llvm::IRBuilder<true, llvm::ConstantFolder, llvm::IRBuilderDefaultInserter<true> >::CreateStore(llvm::Value*, llvm::Value*, bool) () from ../.prefix/lib/librustc_llvm-4e7c5e5c.so
#5  0x00007ffff638e231 in trans::base::store_ty::hf14af618d930153d6Xg () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#6  0x00007ffff63ab6fa in trans::datum::Datum$LT$$u27$tcx$C$$u20$K$GT$::store_to::h12809165441713641485 ()
   from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#7  0x00007ffff63f9209 in trans::datum::Datum$LT$$u27$tcx$C$$u20$Rvalue$GT$::to_appropriate_datum::hdd0e47681bbbddc04Kv ()
   from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#8  0x00007ffff639104e in trans::callee::trans_arg_datum::h1aa473e4a10d26b9eyo () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#9  0x00007ffff63ef3d4 in trans::callee::trans_args::h20f3dc3fe15308e9bpo () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#10 0x00007ffff64672ed in trans::expr::trans_overloaded_op::hf050f93f7fe7de7aPeC ()
   from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#11 0x00007ffff6476da3 in trans::expr::deref_once::h50a37e3af4d668f4cFC () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#12 0x00007ffff6390a71 in trans::expr::trans::h518023c1165f76a7EfA () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#13 0x00007ffff63eedb1 in trans::callee::trans_args::h20f3dc3fe15308e9bpo () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#14 0x00007ffff63f4671 in trans::callee::trans_call_inner::h5004922882929914063 ()
   from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#15 0x00007ffff645a075 in trans::expr::trans_rvalue_dps_unadjusted::h3e4f1c626cf22366Y7A ()
   from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#16 0x00007ffff642d3cd in trans::expr::trans_into::h5aace87d45cda18fl9z () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#17 0x00007ffff63b12e7 in trans::controlflow::trans_block::h83622bcb20d2285cw3u ()
   from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#18 0x00007ffff63afbb7 in trans::base::trans_closure::h1623429c4a8ffbebjHh () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#19 0x00007ffff63b18db in trans::base::trans_fn::hc7a5cef88d3731c91Rh () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#20 0x00007ffff63b4c37 in trans::base::trans_item::h1a43519c5bd06690dgi () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#21 0x00007ffff63c2d08 in trans::base::trans_crate::hadadeed95ebf196004i () from ../.prefix/lib/librustc_trans-4e7c5e5c.so
#22 0x00007ffff7affd6b in driver::phase_4_translate_to_llvm::h7a1973fec1c0688cnOa ()
   from ../.prefix/lib/librustc_driver-4e7c5e5c.so
#23 0x00007ffff7ad8137 in driver::compile_input::h020e6002d10336a4Qba () from ../.prefix/lib/librustc_driver-4e7c5e5c.so
#24 0x00007ffff7b91d62 in run_compiler::he8fc8a587311be2465b () from ../.prefix/lib/librustc_driver-4e7c5e5c.so
#25 0x00007ffff7b8f5b3 in boxed::F.FnBox$LT$A$GT$::call_box::h13553308903082253150 ()
   from ../.prefix/lib/librustc_driver-4e7c5e5c.so
#26 0x00007ffff7b8eb7a in rt::unwind::try::try_fn::h3807368746205497482 () from ../.prefix/lib/librustc_driver-4e7c5e5c.so
#27 0x00007ffff7623069 in rust_try_inner () from ../.prefix/lib/libstd-4e7c5e5c.so
#28 0x00007ffff7623056 in rust_try () from ../.prefix/lib/libstd-4e7c5e5c.so
#29 0x00007ffff7b8ee15 in boxed::F.FnBox$LT$A$GT$::call_box::h15011448770904444204 ()
   from ../.prefix/lib/librustc_driver-4e7c5e5c.so
#30 0x00007ffff75aee62 in sys::thread::Thread::new::thread_start::h8bc03e8ed99cab4d1Iv () from ../.prefix/lib/libstd-4e7c5e5c.so
#31 0x00007ffff1666b50 in start_thread (arg=<optimized out>) at pthread_create.c:304
#32 0x00007ffff720e95d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:112
#33 0x0000000000000000 in ?? ()
