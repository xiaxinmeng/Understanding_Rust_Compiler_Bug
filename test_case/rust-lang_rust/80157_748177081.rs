
#0  0x00007ffff00fa55e in llvm::SelectionDAGBuilder::visitInlineAsm(llvm::CallBase const&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#1  0x00007ffff00d9867 in llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#2  0x00007ffff01994fe in llvm::SelectionDAGISel::SelectBasicBlock(llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, true>, llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, true>, bool&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#3  0x00007ffff0198793 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#4  0x00007ffff0195926 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#5  0x00007ffff1eb4a77 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#6  0x00007fffefcfe78e in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#7  0x00007fffefad8fd2 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#8  0x00007fffefadf9e3 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#9  0x00007fffefad9a2a in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/../lib/libLLVM-11-rust-1.50.0-nightly.so
#10 0x00007ffff3ea8ee0 in LLVMRustWriteOutputFile ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#11 0x00007ffff3da5cdc in rustc_codegen_llvm::back::write::write_output_file ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#12 0x00007ffff3daa74f in rustc_codegen_llvm::back::write::codegen ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#13 0x00007ffff3dcc4db in rustc_codegen_ssa::back::write::finish_intra_module_work ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#14 0x00007ffff3dc684f in rustc_codegen_ssa::back::write::execute_work_item ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4--Type <RET> for more, q to quit, c to continue without paging--
f0cc9f50e53f0ba.so
#15 0x00007ffff3cab4ff in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#16 0x00007ffff3cb125d in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
   from /home/eric/.rustup/toolchains/nightly-2020-12-01-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-4f0cc9f50e53f0ba.so
#17 0x00007ffff313262a in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once ()
    at /rustc/b7ebc6b0c1ba3c27ebb17c0b496ece778ef11e18/library/alloc/src/boxed.rs:1327
#18 <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once ()
    at /rustc/b7ebc6b0c1ba3c27ebb17c0b496ece778ef11e18/library/alloc/src/boxed.rs:1327
#19 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:71
#20 0x00007ffff2e6e6db in start_thread (arg=0x7fffe51ff700) at pthread_create.c:463
#21 0x00007ffff278ba3f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
