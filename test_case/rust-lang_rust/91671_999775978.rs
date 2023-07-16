
Thread 2 received signal SIGSEGV, Segmentation fault.
[Switching to Thread 167404.167414]
0x00007f34b5f640c9 in selectCopy(llvm::MachineInstr&, llvm::TargetInstrInfo const&, llvm::MachineRegisterInfo&, llvm::TargetRegisterInfo const&, llvm::RegisterBankInfo const&) () from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.56.0-nightly.so
(rr) bt
#0  0x00007f34b5f640c9 in selectCopy(llvm::MachineInstr&, llvm::TargetInstrInfo const&, llvm::MachineRegisterInfo&, llvm::TargetRegisterInfo const&, llvm::RegisterBankInfo const&) () from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.56.0-nightly.so
#1  0x00007f34b4c59dae in llvm::InstructionSelect::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.56.0-nightly.so
#2  0x00007f34b45d3fde in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.56.0-nightly.so
#3  0x00007f34b4370429 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.56.0-nightly.so
#4  0x00007f34b43773e3 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.56.0-nightly.so
#5  0x00007f34b4370c90 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-13-rust-1.56.0-nightly.so
#6  0x00007f34b9b8c268 in LLVMRustWriteOutputFile ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-f66b15f11dec651a.so
#7  0x00007f34b9b1304f in rustc_codegen_llvm::back::write::write_output_file ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-f66b15f11dec651a.so
#8  0x00007f34b9b15f70 in rustc_codegen_llvm::back::write::codegen ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-f66b15f11dec651a.so
#9  0x00007f34b9b2146b in rustc_codegen_ssa::back::write::finish_intra_module_work ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-f66b15f11dec651a.so
#10 0x00007f34b9b1ad84 in rustc_codegen_ssa::back::write::execute_work_item ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-f66b15f11dec651a.so
#11 0x00007f34b9b5cd3c in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-f66b15f11dec651a.so
#12 0x00007f34b9b7931c in core::ops::function::FnOnce::call_once{{vtable.shim}} ()
   from /home/pnkfelix/.rustup/toolchains/nightly-2021-08-22-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-f66b15f11dec651a.so
#13 0x00007f34b77a0fe3 in alloc::boxed::{impl#44}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> ()
    at /rustc/d3e2578c31688619ddc0a10ddf8543bf4ebcba5b/library/alloc/src/boxed.rs:1636
#14 alloc::boxed::{impl#44}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> ()
    at /rustc/d3e2578c31688619ddc0a10ddf8543bf4ebcba5b/library/alloc/src/boxed.rs:1636
#15 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:106
#16 0x00007f34b7539927 in start_thread (arg=<optimized out>) at pthread_create.c:435
#17 0x00007f34b75c99e4 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:100
