
opt: /export/scratch/laden/rust/src/llvm/lib/Analysis/ValueTracking.cpp:3202: bool llvm::isKnownNonNull(const llvm::Value*): Assertion `V->getType()->isPointerTy() && "V must be pointer type"' failed.
#0 0x0000562780420ee5 llvm::sys::PrintStackTrace(llvm::raw_ostream&) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x1469ee5)
#1 0x000056278041eb6e llvm::sys::RunSignalHandlers() (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x1467b6e)
#2 0x000056278041ecbc SignalHandler(int) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x1467cbc)
#3 0x00007f5b576da0c0 __restore_rt (/lib/x86_64-linux-gnu/libpthread.so.0+0x110c0)
#4 0x00007f5b566b4fdf gsignal (/lib/x86_64-linux-gnu/libc.so.6+0x32fdf)
#5 0x00007f5b566b640a abort (/lib/x86_64-linux-gnu/libc.so.6+0x3440a)
#6 0x00007f5b566ade47 (/lib/x86_64-linux-gnu/libc.so.6+0x2be47)
#7 0x00007f5b566adef2 (/lib/x86_64-linux-gnu/libc.so.6+0x2bef2)
#8 0x000056277fc476b0 llvm::isKnownNonNull(llvm::Value const*) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0xc906b0)
#9 0x000056277fc47782 llvm::isKnownNonNullAt(llvm::Value const*, llvm::Instruction const*, llvm::DominatorTree const*) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0xc90782)
#10 0x00005627804bf4d0 (anonymous namespace)::PromoteMem2Reg::run() (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x15084d0)
#11 0x00005627804c094b llvm::PromoteMemToReg(llvm::ArrayRef<llvm::AllocaInst*>, llvm::DominatorTree&, llvm::AliasSetTracker*, llvm::AssumptionCache*) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x150994b)
#12 0x000056278033e1fb llvm::SROA::promoteAllocas(llvm::Function&) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x13871fb)
#13 0x0000562780355e6d llvm::SROA::runImpl(llvm::Function&, llvm::DominatorTree&, llvm::AssumptionCache&) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x139ee6d)
#14 0x00005627803570dd llvm::sroa::SROALegacyPass::runOnFunction(llvm::Function&) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x13a00dd)
#15 0x000056277fffae33 llvm::FPPassManager::runOnFunction(llvm::Function&) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x1043e33)
#16 0x000056277fffb1eb llvm::FPPassManager::runOnModule(llvm::Module&) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x10441eb)
#17 0x000056277fffb521 llvm::legacy::PassManagerImpl::run(llvm::Module&) (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x1044521)
#18 0x000056277f3e5492 main (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x42e492)
#19 0x00007f5b566a22b1 __libc_start_main (/lib/x86_64-linux-gnu/libc.so.6+0x202b1)
#20 0x000056277f43670a _start (/scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt+0x47f70a)
Stack dump:
0.      Program arguments: /scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt bad.ll -S -sroa
1.      Running pass 'Function Pass Manager' on module 'bad.ll'.
2.      Running pass 'SROA' on function '@fail_sroa'
[1]    21755 abort      /scratch/laden/rust/build/x86_64-unknown-linux-gnu/llvm/bin/opt bad.ll -S
