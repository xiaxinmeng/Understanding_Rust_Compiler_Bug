
Thread 2 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fb2bfeed0 (LWP 6685)]
getCopyFromParts(llvm::SelectionDAG&, llvm::SDLoc, llvm::SDValue const*, unsigned int, llvm::MVT, llvm::EVT, llvm::Value const*, llvm::ISD::NodeType) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/include/llvm/CodeGen/SelectionDAGNodes.h:706
706         return ValueList[ResNo];
(gdb) bt
#0  getCopyFromParts(llvm::SelectionDAG&, llvm::SDLoc, llvm::SDValue const*, unsigned int, llvm::MVT, llvm::EVT, llvm::Value const*, llvm::ISD::NodeType) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/include/llvm/CodeGen/SelectionDAGNodes.h:706
#1  0x0000007fb3d9aea8 in llvm::SelectionDAGISel::LowerArguments(llvm::Function const&) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:7533
warning: Could not find DWO CU CMakeFiles/LLVMSelectionDAG.dir/SelectionDAGISel.cpp.dwo(0x63d611a14ea36bbf) referenced by CU at offset 0xb7b9 [in module /home/infinity0/vroot/usr/lib/debug/.build-id/38/1a430b7891664d86e253c9a285580437205637.debug]
#2  0x0000007fb3db1ba0 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:1221
#3  0x0000007fb3db293c in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:502
warning: Could not find DWO CU CMakeFiles/LLVMCore.dir/LegacyPassManager.cpp.dwo(0xe2d0d448fbcb5d61) referenced by CU at offset 0x9601 [in module /home/infinity0/vroot/usr/lib/debug/.build-id/38/1a430b7891664d86e253c9a285580437205637.debug]
#4  0x0000007fb3a60dc4 in llvm::FPPassManager::runOnFunction(llvm::Function&) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/lib/IR/LegacyPassManager.cpp:1550
#5  0x0000007fb3a610fc in llvm::FPPassManager::runOnModule(llvm::Module&) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/lib/IR/LegacyPassManager.cpp:1571
#6  0x0000007fb3a6142c in llvm::legacy::PassManagerImpl::run(llvm::Module&) () at /home/infinity0/llvm-toolchain-3.8-3.8.1/lib/IR/LegacyPassManager.cpp:1627
#7  0x0000007fb60ad954 in LLVMRustWriteOutputFile (Target=<optimized out>, PMR=<optimized out>, M=<optimized out>, path=<optimized out>, FileType=<optimized out>) at ./src/rustllvm/PassWrapper.cpp:310
#8  0x0000007fb7487954 in rustc_trans::back::write::write_output_file::h6987760832c57d2e () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_trans-39b92f95.so
#9  0x0000007fb74898ec in rustc_trans::back::write::optimize_and_codegen::_$u7b$$u7b$closure$u7d$$u7d$::h965e55032de68f69 () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_trans-39b92f95.so
#10 0x0000007fb74902a0 in rustc_trans::back::write::execute_work_item::h46f7bca878f71bc1 () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_trans-39b92f95.so
#11 0x0000007fb748a614 in rustc_trans::back::write::run_passes::h4610f818f672d1a8 () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_trans-39b92f95.so
#12 0x0000007fb7e81298 in rustc_driver::driver::phase_5_run_llvm_passes::h283134306ba7308a () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-39b92f95.so
#13 0x0000007fb7e3d1b8 in rustc_driver::driver::compile_input::hdfe4405d66704c31 () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-39b92f95.so
#14 0x0000007fb7e2c114 in rustc_driver::run_compiler::h581448fb74257353 () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-39b92f95.so
#15 0x0000007fb7e294f8 in std::panicking::try::call::hf081e8ea5e252d1a () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-39b92f95.so
#16 0x0000007fb7b54f8c in __rust_try () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/libstd-39b92f95.so
#17 0x0000007fb7b54f24 in __rust_maybe_catch_panic () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/libstd-39b92f95.so
#18 0x0000007fb7e29f60 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h2d5dcb354b3ff8db () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-39b92f95.so
#19 0x0000007fb7b481a4 in std::sys::thread::Thread::new::thread_start::hf2eed4b6f7149599 () from /home/infinity0/rustc-1.11.0+dfsg1/aarch64-unknown-linux-gnu/stage2/bin/../lib/libstd-39b92f95.so
#20 0x0000007fb585a018 in start_thread () from /lib/aarch64-linux-gnu/libpthread.so.0
#21 0x0000007fb79cdd18 in ?? () from /lib/aarch64-linux-gnu/libc.so.6
