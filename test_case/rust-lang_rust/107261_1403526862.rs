
0x00007ffff2136edf in llvm::RAGreedy::calculateRegionSplitCost(llvm::LiveInterval const&, llvm::AllocationOrder&, llvm::BlockFrequency&, unsigned int&, bool) () from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
=> 0x00007ffff2136edf <_ZN4llvm8RAGreedy24calculateRegionSplitCostERKNS_12LiveIntervalERNS_15AllocationOrderERNS_14BlockFrequencyERjb+4059>:	48 8b 49 10	mov    0x10(%rcx),%rcx
(gdb) bt
#0  0x00007ffff2136edf in llvm::RAGreedy::calculateRegionSplitCost(llvm::LiveInterval const&, llvm::AllocationOrder&, llvm::BlockFrequency&, unsigned int&, bool) () from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#1  0x00007ffff213570a in llvm::RAGreedy::tryRegionSplit(llvm::LiveInterval const&, llvm::AllocationOrder&, llvm::SmallVectorImpl<llvm::Register>&)
    () from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#2  0x00007ffff21347c4 in llvm::RAGreedy::trySplit(llvm::LiveInterval const&, llvm::AllocationOrder&, llvm::SmallVectorImpl<llvm::Register>&, llvm::SmallSet<llvm::Register, 16u, std::less<llvm::Register> > const&) ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#3  0x00007ffff20d9481 in non-virtual thunk to llvm::RAGreedy::selectOrSplit(llvm::LiveInterval const&, llvm::SmallVectorImpl<llvm::Register>&) ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#4  0x00007ffff213020c in llvm::RegAllocBase::allocatePhysRegs() ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#5  0x00007ffff2197f30 in llvm::RAGreedy::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#6  0x00007ffff2387461 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#7  0x00007ffff238692f in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#8  0x00007ffff220b29e in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/libLLVM-15-rust-1.67.0-nightly.so
#9  0x00007ffff6357582 in LLVMRustWriteOutputFile ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/librustc_driver-e0c8ab3d159f8c4e.so
#10 0x00007ffff6356f45 in rustc_codegen_llvm::back::write::write_output_file ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/librustc_driver-e0c8ab3d159f8c4e.so
#11 0x00007ffff6354e97 in rustc_codegen_llvm::back::write::codegen ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/librustc_driver-e0c8ab3d159f8c4e.so
#12 0x00007ffff6385f9d in rustc_codegen_ssa::back::write::finish_intra_module_work::<rustc_codegen_llvm::LlvmCodegenBackend> ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/librustc_driver-e0c8ab3d159f8c4e.so
#13 0x00007ffff6384ecc in rustc_codegen_ssa::back::write::execute_work_item::<rustc_codegen_llvm::LlvmCodegenBackend> ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/librustc_driver-e0c8ab3d159f8c4e.so
#14 0x00007ffff6383d2e in std::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::spawn_named_thread<rustc_codegen_ssa::back::write::spawn_work<rustc_codegen_llvm::LlvmCodegenBackend>::{closure#0}, ()>::{closure#0}, ()> () from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/librustc_driver-e0c8ab3d159f8c4e.so
#15 0x00007ffff62898f4 in <<std::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::spawn_named_thread<rustc_codegen_ssa::back::write::spawn_work<rustc_codegen_llvm::LlvmCodegenBackend>::{closure#0}, ()>::{closure#0}, ()>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} ()
   from /home/cactus/.rustup/toolchains/nightly-2022-12-09-x86_64-unknown-linux-gnu/lib/librustc_driver-e0c8ab3d159f8c4e.so
#16 0x00007ffff7c45b33 in alloc::boxed::{impl#45}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> ()
    at library/alloc/src/boxed.rs:2000
#17 alloc::boxed::{impl#45}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Glob--Type <RET> for more, q to quit, c to continue without paging--
al> () at library/alloc/src/boxed.rs:2000
#18 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
#19 0x00007ffff3894b43 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
#20 0x00007ffff3926a00 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
