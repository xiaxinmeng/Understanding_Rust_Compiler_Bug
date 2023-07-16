
Not a vector MVT!
UNREACHABLE executed at .../rust/src/llvm/include/llvm/Support/MachineValueType.h:521!

#0  0x00007ffff729f428 in __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:54
#1  0x00007ffff72a102a in __GI_abort () at abort.c:89
#2  0x00007fffedb74f3a in llvm::llvm_unreachable_internal(char const*, char const*, unsigned int) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fffebbb35ac in llvm::MVT::getVectorNumElements() const ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fffebd009af in llvm::X86TTIImpl::getShuffleCost(llvm::TargetTransformInfo::ShuffleKind, llvm::Type*, int, llvm::Type*) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fffebd03402 in llvm::X86TTIImpl::getArithmeticReductionCost(unsigned int, llvm::Type*, bool) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fffed6c703a in llvm::TargetTransformInfo::getArithmeticReductionCost(unsigned int, llvm::Type*, bool) const ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fffec626238 in tryToVectorizeHorReductionOrInstOperands(llvm::PHINode*, llvm::Instruction*, llvm::BasicBlock*, llvm::slpvectorizer::BoUpSLP&, llvm::TargetTransformInfo*, 
llvm::function_ref<bool (llvm::Instruction*, llvm::slpvectorizer::BoUpSLP&)>) [clone .constprop.1301] ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fffec627019 in llvm::SLPVectorizerPass::vectorizeRootInstruction(llvm::PHINode*, llvm::Value*, llvm::BasicBlock*, llvm::slpvectorizer::BoUpSLP&, llvm::TargetTransformInfo*) () from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fffec627e68 in llvm::SLPVectorizerPass::vectorizeChainsInBlock(llvm::BasicBlock*, llvm::slpvectorizer::BoUpSLP&) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fffec628762 in llvm::SLPVectorizerPass::runImpl(llvm::Function&, llvm::ScalarEvolution*, llvm::TargetTransformInfo*, llvm::TargetLibraryInfo*, llvm::AAResults*, llvm::LoopInfo*, llvm::DominatorTree*, llvm::AssumptionCache*, llvm::DemandedBits*, llvm::OptimizationRemarkEmitter*) [clone .part.1286] ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fffec629f89 in (anonymous namespace)::SLPVectorizer::runOnFunction(llvm::Function&) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fffed8e2acc in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fffed8e2cac in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fffed8e3dc9 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fffed836ece in LLVMRunPassManager ()
   from .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007fffeb95aca6 in rustc_codegen_llvm::back::lto::run_pass_manager::hc6c0f595af3c1fd5 (cgcx=0x7fffe37fa280, module=0x7fffe37f9a80, config=0x55555e4d0760,
    thin=<optimized out>) at src/librustc_codegen_llvm/back/lto.rs:573
#17 0x00007fffeb95b441 in rustc_codegen_llvm::back::lto::optimize_thin_module::h5d05c4879f335b44 (thin_module=0x7fffe37f9fd8, cgcx=0x7fffe37fa280, timeline=0x7fffe37fa120)
    at src/librustc_codegen_llvm/back/lto.rs:786
#18 0x00007fffeba795c5 in _$LT$rustc_codegen_ssa..back..lto..LtoModuleCodegen$LT$B$GT$$GT$::optimize::hc84145a7c757d774 () at src/librustc_codegen_llvm/lib.rs:208
#19 0x00007fffeb9738bd in rustc_codegen_ssa::back::write::execute_lto_work_item::h9edd0f4b18335454 (cgcx=<optimized out>, module_config=0x55555e4d0760, timeline=<optimized out>,
    module=<optimized out>) at .../rust/src/librustc_codegen_ssa/back/write.rs:854
#20 rustc_codegen_ssa::back::write::execute_work_item::h9dade10f4e169e8f (cgcx=0x7fffe37fa280,
    work_item=<unknown type in .../rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so, CU 0x236ce2, DIE 0x283800>, timeline=0x7fffe37fa120) at .../rust/src/librustc_codegen_ssa/back/write.rs:702
#21 0x00007fffeb939b08 in rustc_codegen_ssa::back::write::spawn_work::_$u7b$$u7b$closure$u7d$$u7d$::h43f19184ef9ba023 ()
    at .../rust/src/librustc_codegen_ssa/back/write.rs:1562
#22 std::sys_common::backtrace::__rust_begin_short_backtrace::hb56b30729901a64e (f=...) at .../rust/src/libstd/sys_common/backtrace.rs:137
#23 0x00007fffeb93a876 in std::thread::Builder::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::heaa344e450ebecf9 ()
    at .../rust/src/libstd/thread/mod.rs:479
#24 _$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h9c20c58d97fbd899 (self=..., _args=<optimized out>)
    at .../rust/src/libstd/panic.rs:319
#25 std::panicking::try::do_call::h6636d1dd0b0de513 (data=<optimized out>) at .../rust/src/libstd/panicking.rs:307
#26 0x00007ffff76d12da in __rust_maybe_catch_panic (f=0x0, data=0x4558 <error: Cannot access memory at address 0x4558>, data_ptr=0x7fffe37fa618, vtable_ptr=0x7fffe37fa620)
    at src/libpanic_unwind/lib.rs:102
#27 0x00007fffeb93a77a in std::panicking::try::h7fb1e4da3a8d54ae (f=...) at .../rust/src/libstd/panicking.rs:286
#28 0x00007fffeba72f44 in std::panic::catch_unwind::h3be20d126e54c91f (f=...) at .../rust/src/libstd/panic.rs:398
#29 std::thread::Builder::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h1552c69c06bb502d () at .../rust/src/libstd/thread/mod.rs:478
#30 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h3d9baf442970a7cf (self=0x7fffdc001130, args=<optimized out>) at .../rust/src/liballoc/boxed.rs:673
#31 0x00007ffff76c206e in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h0f8d9ed6ff872497 (self=..., args=<optimized out>) at .../rust/src/liballoc/boxed.rs:683
#32 std::sys_common::thread::start_thread::hdf2774f5b4d938b0 (main=0x7fffdc0db6e0 "0\021") at src/libstd/sys_common/thread.rs:24
#33 0x00007ffff76be5b6 in std::sys::unix::thread::Thread::new::thread_start::h7623e09fa06f2a88 (main=0x4558) at src/libstd/sys/unix/thread.rs:91
#34 0x00007ffff12b76ba in start_thread (arg=0x7fffe37fb700) at pthread_create.c:333
#35 0x00007ffff737141d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
