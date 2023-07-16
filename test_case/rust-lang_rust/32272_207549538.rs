 rust
rustc.bin: /src/llvm/lib/Transforms/Utils/PromoteMemoryToRegister.cpp:531: void {anonymous}::PromoteMem2Reg::run(): Assertion `isAllocaPromotable(AI) && "Cannot promote non-promotable alloca!"' failed.

Program received signal SIGABRT, Aborted.
[Switching to Thread 0xb04bd2b0 (LWP 28697)]
__libc_do_syscall () at ../ports/sysdeps/unix/sysv/linux/arm/libc-do-syscall.S:44
44      ../ports/sysdeps/unix/sysv/linux/arm/libc-do-syscall.S: No such file or directory.
(gdb) bt
#0  __libc_do_syscall () at ../ports/sysdeps/unix/sysv/linux/arm/libc-do-syscall.S:44
#1  0xb661bf0e in __GI_raise (sig=sig@entry=6) at ../nptl/sysdeps/unix/sysv/linux/raise.c:56
#2  0xb661e766 in __GI_abort () at abort.c:89
#3  0xb6617150 in __assert_fail_base (fmt=0x1 <error: Cannot access memory at address 0x1>, assertion=0xb499aa18 "isAllocaPromotable(AI) && \"Cannot promote non-promotable alloca!\"", assertion@entry=0x0, 
    file=0xb499a8c8 "/src/llvm/lib/Transforms/Utils/PromoteMemoryToRegister.cpp", file@entry=0xb04bd2b0 "\001", line=531, 
    line@entry=3060637868, function=function@entry=0xb4997bf0 <(anonymous namespace)::PromoteMem2Reg::run()::__PRETTY_FUNCTION__> "void {anonymous}::PromoteMem2Reg::run()") at assert.c:92
#4  0xb66171e6 in __GI___assert_fail (assertion=0x0, file=0xb04bd2b0 "\001", line=3060637868, 
    function=0xb4997bf0 <(anonymous namespace)::PromoteMem2Reg::run()::__PRETTY_FUNCTION__> "void {anonymous}::PromoteMem2Reg::run()") at assert.c:101
#5  0xb3a6d8c0 in (anonymous namespace)::PromoteMem2Reg::run() () from rust/bin/../lib/librustc_llvm-9026086f.so
#6  0xb3a6db3c in llvm::PromoteMemToReg(llvm::ArrayRef<llvm::AllocaInst*>, llvm::DominatorTree&, llvm::AliasSetTracker*, llvm::AssumptionCache*) ()
   from rust/bin/../lib/librustc_llvm-9026086f.so
#7  0xb38af710 in llvm::SROA::promoteAllocas(llvm::Function&) () from rust/bin/../lib/librustc_llvm-9026086f.so
#8  0xb38c57d8 in llvm::SROA::runImpl(llvm::Function&, llvm::DominatorTree&, llvm::AssumptionCache&) () from rust/bin/../lib/librustc_llvm-9026086f.so
#9  0xb38c5eb8 in llvm::sroa::SROALegacyPass::runOnFunction(llvm::Function&) () from rust/bin/../lib/librustc_llvm-9026086f.so
#10 0xb3fa8bcc in llvm::FPPassManager::runOnFunction(llvm::Function&) () from rust/bin/../lib/librustc_llvm-9026086f.so
#11 0xb3b1195c in (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) () from rust/bin/../lib/librustc_llvm-9026086f.so
#12 0xb3fa93bc in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from rust/bin/../lib/librustc_llvm-9026086f.so
#13 0xb3de0994 in LLVMRunPassManager () from rust/bin/../lib/librustc_llvm-9026086f.so
#14 0xb60adf38 in fnfn () at src/librustc_trans/back/lto.rs:163
#15 0xb60ada84 in rustc_trans::util::common::time<u32,closure> (do_it=false, what=..., f=...) at src/librustc/util/common.rs:38
#16 0xb60a74d4 in rustc_trans::back::lto::run (sess=0xb04baf10, llmod=0x965cae08, tm=0x96900018, reachable=..., config=0xb04b84e8, name_extra=..., output_names=0xb04b8488) at src/librustc_trans/back/lto.rs:162
#17 0xb60ca4a8 in fnfn () at src/librustc_trans/back/write.rs:504
#18 0xb60c9f44 in rustc_trans::util::common::time<(),closure> (do_it=false, what=..., f=...) at src/librustc/util/common.rs:38
#19 0xb60c73fc in rustc_trans::back::write::optimize_and_codegen (cgcx=0xb04b88a8, mtrans=..., config=..., name_extra=..., output_names=...) at src/librustc_trans/back/write.rs:503
#20 0xb60d10f4 in rustc_trans::back::write::execute_work_item (cgcx=0xb04b88a8, work_item=...) at src/librustc_trans/back/write.rs:875
#21 0xb60cfcd8 in rustc_trans::back::write::run_work_singlethreaded (sess=0xb04baf10, reachable=..., work_items=...) at src/librustc_trans/back/write.rs:888
#22 0xb60cc454 in rustc_trans::back::write::run_passes (sess=0xb04baf10, trans=0xb04b97a0, output_types=0xb04bb228, crate_output=0xb04b97e8) at src/librustc_trans/back/write.rs:712
#23 0xb6e8c418 in fnfn () at src/librustc_driver/driver.rs:981
#24 0xb6e8bf54 in rustc_driver::util::common::time<(),closure> (do_it=false, what=..., f=...) at src/librustc/util/common.rs:38
#25 0xb6cb6ec4 in rustc_driver::driver::phase_5_run_llvm_passes (sess=0xb04baf10, trans=0xb04b97a0, outputs=0xb04b97e8) at src/librustc_driver/driver.rs:979
#26 0xb6c008c0 in rustc_driver::driver::compile_input (sess=0xb04baf10, cstore=0xafb14bc8, cfg=..., input=0xb04bb6a8, outdir=0xb04bb794, output=0xb04bb788, addl_plugins=..., control=0xb04ba4ec)
    at src/librustc_driver/driver.rs:222
#27 0xb6be3e98 in rustc_driver::run_compiler (args=..., callbacks=...) at src/librustc_driver/lib.rs:213
#28 0xb6be07b8 in fnfn () at src/librustc_driver/lib.rs:132
#29 0xb6bde8d0 in fnfn () at src/librustc_driver/lib.rs:997
#30 0xb6bde3a8 in std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h71e29f932f68a9c9 () at src/libstd/sys/common/mutex.rs:28
#31 0xb6bde340 in rustc_driver::sys_common::unwind::try::try_fn<closure> (opt_closure=0xb04bcb08 "") at src/libstd/sys/common/unwind/mod.rs:127
#32 0xb6857d6c in __rust_try () from rust/bin/../lib/libstd-9026086f.so
#33 0xb6857a54 in fnfn (s=0xb04bd7a8) at src/libstd/sys/common/unwind/mod.rs:148
#34 0xb6857950 in std::thread::local::{{impl}}::with<closure,core::result::Result<(), Box<Any>>> (self=0xb69bcab0 <std::panicking::PANIC_COUNT::hb97300d99929c675>, f=...) at src/libstd/thread/local.rs:211
#35 0xb6857858 in std::sys_common::unwind::inner_try (f=0xb6bde308 <rustc_driver::sys_common::unwind::try::try_fn<closure>>, data=0xb04bcb08 "") at src/libstd/sys/common/unwind/mod.rs:133
#36 0xb6bde2a8 in rustc_driver::sys_common::unwind::try<closure> (f=...) at src/libstd/sys/common/unwind/mod.rs:123
#37 0xb6bde154 in std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::h98fc4c71843fb136 () at src/libstd/sys/common/mutex.rs:28
#38 0xb6bdeba8 in rustc_driver::boxed::{{impl}}::call_box (self=0x7f571590, args=0) at src/liballoc/boxed.rs:541
#39 0xb6846a38 in std::boxed::{{impl}}::call_once (self=..., args=0) at src/liballoc/boxed.rs:550
#40 0xb6854fb8 in std::sys_common::thread::start_thread (main=0x7f571650) at src/libstd/sys/common/thread.rs:23
#41 0xb6892798 in std::sys::thread::{{impl}}::new::thread_start (main=0x7f571650) at src/libstd/sys/unix/thread.rs:74
#42 0xb07dafbc in start_thread (arg=0xb04bd2b0) at pthread_create.c:314
#43 0xb668920c in ?? () at ../ports/sysdeps/unix/sysv/linux/arm/nptl/../clone.S:92 from /lib/arm-linux-gnueabihf/libc.so.6
