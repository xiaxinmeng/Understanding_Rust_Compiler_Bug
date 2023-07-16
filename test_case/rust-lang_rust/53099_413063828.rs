
#0  0x00007f9c54b59ea6 in llvm::CallSiteBase<llvm::Function const, llvm::BasicBlock const, llvm::Value const, llvm::User const, llvm::Use const, llvm::Instruction const, llvm::Call
Inst const, llvm::InvokeInst const, llvm::Use const*>::getCalledValue (this=<synthetic pointer>) at /home/brook/rust/src/llvm/include/llvm/ADT/PointerIntPair.h:155
#1  llvm::CallSiteBase<llvm::Function const, llvm::BasicBlock const, llvm::Value const, llvm::User const, llvm::Use const, llvm::Instruction const, llvm::CallInst const, llvm::Invo
keInst const, llvm::Use const*>::getCalledFunction (this=<synthetic pointer>) at /home/brook/rust/src/llvm/include/llvm/IR/CallSite.h:108
#2  llvm::NVPTXTargetLowering::getPrototype[abi:cxx11](llvm::DataLayout const&, llvm::Type*, std::vector<llvm::TargetLoweringBase::ArgListEntry, std::allocator<llvm::TargetLowering
Base::ArgListEntry> > const&, llvm::SmallVectorImpl<llvm::ISD::OutputArg> const&, unsigned int, llvm::ImmutableCallSite) const ()
    at /home/brook/rust/src/llvm/lib/Target/NVPTX/NVPTXISelLowering.cpp:1276
#3  0x00007f9c54b5c7de in llvm::NVPTXTargetLowering::LowerCall(llvm::TargetLowering::CallLoweringInfo&, llvm::SmallVectorImpl<llvm::SDValue>&) const ()
    at /home/brook/rust/src/llvm/lib/Target/NVPTX/NVPTXISelLowering.cpp:1661
#4  0x00007f9c55545cbb in llvm::TargetLowering::LowerCallTo (this=0x564cefae0618, CLI=...) at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:8537
#5  0x00007f9c554d19c5 in llvm::DAGTypeLegalizer::ExpandIntRes_XMULO (this=this@entry=0x7f9c5348db90, N=N@entry=0x7f9c300ff288, Lo=..., Hi=...)
    at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/LegalizeIntegerTypes.cpp:2778
#6  0x00007f9c554db6c5 in llvm::DAGTypeLegalizer::ExpandIntegerResult (this=this@entry=0x7f9c5348db90, N=N@entry=0x7f9c300ff288, ResNo=ResNo@entry=0)
    at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/LegalizeIntegerTypes.cpp:1477
#7  0x00007f9c554e5940 in llvm::DAGTypeLegalizer::run() () at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/LegalizeTypes.cpp:257
#8  0x00007f9c554e5ccd in llvm::SelectionDAG::LegalizeTypes (this=<optimized out>) at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/LegalizeTypes.h:175
#9  0x00007f9c555bc059 in llvm::SelectionDAGISel::CodeGenAndEmitDAG() () at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:770
#10 0x00007f9c555bc7a0 in llvm::SelectionDAGISel::SelectBasicBlock (this=this@entry=0x7f9c3001a790, Begin=..., Begin@entry=..., End=..., End@entry=...,
    HadTailCall=<error reading variable>) at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:667
#11 0x00007f9c555c1311 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) () at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:1733
#12 0x00007f9c555c37a9 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) () at /home/brook/rust/src/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:469
#13 0x00007f9c557a4143 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) () at /home/brook/rust/src/llvm/lib/CodeGen/MachineFunctionPass.cpp:61
#14 0x00007f9c5615b548 in llvm::FPPassManager::runOnFunction (this=0x7f9c300a6820, F=...) at /home/brook/rust/src/llvm/lib/IR/LegacyPassManager.cpp:1593
#15 0x00007f9c5615b5c3 in llvm::FPPassManager::runOnModule (this=0x7f9c300a6820, M=...) at /home/brook/rust/src/llvm/lib/IR/LegacyPassManager.cpp:1609
#16 0x00007f9c5615ac70 in (anonymous namespace)::MPPassManager::runOnModule (M=..., this=0x7f9c3000c4e0) at /home/brook/rust/src/llvm/lib/IR/LegacyPassManager.cpp:1669
#17 llvm::legacy::PassManagerImpl::run(llvm::Module&) () at /home/brook/rust/src/llvm/lib/IR/LegacyPassManager.cpp:1774
#18 0x00007f9c5615af49 in llvm::legacy::PassManager::run (this=this@entry=0x7f9c300e8010, M=...) at /home/brook/rust/src/llvm/lib/IR/LegacyPassManager.cpp:1805
#19 0x00007f9c546d4497 in LLVMRustWriteOutputFile (Target=<optimized out>, PMR=0x7f9c300e8010, M=0x564ceac35760,
    Path=0x7f9c300d2eb0 "/tmp/xargo.JwjTnk5yZGqd/target/nvptx64-nvidia-cuda/release/deps/core-61b4d04ef2a9c7b3.1i4jyjk3gxvvvngj.rcgu.o", RustFileType=<optimized out>)
    at /home/brook/rust/src/llvm/include/llvm/IR/Module.h:877
#20 0x00007f9c5463791a in rustc_codegen_llvm::back::write::write_output_file (handler=0x7f9c5348fb60, target=0x564cf1fa74e8, pm=0x80, m=0xffffffffffffffe8,
    output=<error reading variable: access outside bounds of object referenced via synthetic pointer>, file_type=rustc_codegen_llvm::llvm::ffi::ObjectFile)
    at librustc_codegen_llvm/back/write.rs:100
#21 0x00007f9c545facba in rustc_codegen_llvm::back::write::codegen::{{closure}}::{{closure}} (cpm=0x7f9c300e8010) at librustc_codegen_llvm/back/write.rs:804
#22 rustc_codegen_llvm::back::write::codegen::with_codegen (tm=<optimized out>, llmod=<optimized out>, no_builtins=<optimized out>, f=...)
    at librustc_codegen_llvm/back/write.rs:684
#23 0x00007f9c54601478 in rustc_codegen_llvm::back::write::codegen::{{closure}} () at librustc_codegen_llvm/back/write.rs:803
#24 0x00007f9c545ffac7 in rustc::util::common::time_ext (do_it=<optimized out>, sess=..., what=..., f=...) at /home/brook/rust/src/librustc/util/common.rs:163
#25 0x00007f9c54639b4d in rustc_codegen_llvm::back::write::codegen (cgcx=<optimized out>, diag_handler=<optimized out>, module=..., config=<optimized out>,
    timeline=<optimized out>) at librustc_codegen_llvm/back/write.rs:740
#26 0x00007f9c54641314 in rustc_codegen_llvm::back::write::execute_work_item (cgcx=<optimized out>, work_item=..., timeline=0x7f9c5348fcf0)
    at librustc_codegen_llvm/back/write.rs:1404
#27 0x00007f9c5466440b in rustc_codegen_llvm::back::write::spawn_work::{{closure}} () at librustc_codegen_llvm/back/write.rs:2049
#28 std::sys_common::backtrace::__rust_begin_short_backtrace (f=...) at /home/brook/rust/src/libstd/sys_common/backtrace.rs:136
#29 0x00007f9c5462c698 in std::thread::Builder::spawn::{{closure}}::{{closure}} () at /home/brook/rust/src/libstd/thread/mod.rs:409
#30 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once (self=..., _args=<optimized out>) at /home/brook/rust/src/libstd/panic.rs:313
#31 0x00007f9c545ebbd8 in std::panicking::try::do_call (data=<optimized out>) at /home/brook/rust/src/libstd/panicking.rs:310
#32 0x00007f9c60706db1 in __rust_maybe_catch_panic (f=0x7f9c545ebbb0 <std::panicking::try::do_call>, data=0x564cf1fa74e8 "\260e\374\360LV", data_ptr=0x7f9c53490438,
    vtable_ptr=0x7f9c53490440) at libpanic_unwind/lib.rs:105
#33 0x00007f9c545eba5b in std::panicking::try (f=...) at /home/brook/rust/src/libstd/panicking.rs:289
#34 0x00007f9c5462c758 in std::panic::catch_unwind (f=...) at /home/brook/rust/src/libstd/panic.rs:392
#35 0x00007f9c545666df in std::thread::Builder::spawn::{{closure}} () at /home/brook/rust/src/libstd/thread/mod.rs:408
#36 <F as alloc::boxed::FnBox<A>>::call_box (self=0x7f9c380014d0, args=<optimized out>) at /home/brook/rust/src/liballoc/boxed.rs:642
#37 0x00007f9c606f97fc in std::sys_common::thread::start_thread (main=0x564cf0b4af90 "\320\024") at libstd/sys_common/thread.rs:24
#38 0x00007f9c606b9c66 in std::sys::unix::thread::Thread::new::thread_start (main=0x564cf1fa74e8) at libstd/sys/unix/thread.rs:90
#39 0x00007f9c588e2075 in start_thread () from /usr/lib/libpthread.so.0
#40 0x00007f9c6036b53f in clone () from /usr/lib/libc.so.6
