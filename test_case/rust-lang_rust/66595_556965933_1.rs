
(gdb) info threads 
  Id   Target Id                                 Frame 
* 1    Thread 0x7fd470219400 (LWP 20211) "rustc" __GI___pthread_timedjoin_ex (
    threadid=140550382548736, thread_return=0x0, abstime=<optimized out>, block=<optimized out>)
    at pthread_join_common.c:142
  2    Thread 0x7fd46f9ff700 (LWP 20212) "rustc" futex_wait_cancelable (private=<optimized out>, 
    expected=0, futex_word=0x7fd46fe1123c) at ../sysdeps/unix/sysv/linux/futex-internal.h:80
  3    Thread 0x7fd468a0b700 (LWP 20249) "rustc" futex_wait_cancelable (private=<optimized out>, 
    expected=0, futex_word=0x7fd4627f123c) at ../sysdeps/unix/sysv/linux/futex-internal.h:80
  4    Thread 0x7fd45cce3700 (LWP 20250) "rustc" futex_wait_cancelable (private=<optimized out>, 
    expected=0, futex_word=0x7fd4627f1598) at ../sysdeps/unix/sysv/linux/futex-internal.h:80
  5    Thread 0x7fd45b17f700 (LWP 20268) "rustc" 0x00007fd46aa5ffd2 in (anonymous namespace)::DAGCombiner::visitSTORE(llvm::SDNode*) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
(gdb) thread 5
[Switching to thread 5 (Thread 0x7fd45b17f700 (LWP 20268))]
#0  0x00007fd46aa5ffd2 in (anonymous namespace)::DAGCombiner::visitSTORE(llvm::SDNode*) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
(gdb) bt
#0  0x00007fd46aa5ffd2 in (anonymous namespace)::DAGCombiner::visitSTORE(llvm::SDNode*) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#1  0x00007fd46aa0611c in (anonymous namespace)::DAGCombiner::visit(llvm::SDNode*) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#2  0x00007fd46aa02a4c in (anonymous namespace)::DAGCombiner::combine(llvm::SDNode*) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#3  0x00007fd46aa011e3 in llvm::SelectionDAG::Combine(llvm::CombineLevel, llvm::AAResults*, llvm::CodeGenOpt::Level) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#4  0x00007fd46ac382f0 in llvm::SelectionDAGISel::CodeGenAndEmitDAG() ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#5  0x00007fd46ac37b48 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#6  0x00007fd46ac34176 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#7  0x00007fd46c70591d in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#8  0x00007fd46a7fcd9e in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#9  0x00007fd46a60542f in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#10 0x00007fd46a605893 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#11 0x00007fd46a606030 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.41.0-nightly.so
#12 0x00007fd46d07a4ef in LLVMRustWriteOutputFile () from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fd46d027a9c in rustc_codegen_llvm::back::write::write_output_file ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fd46cf17b96 in rustc_codegen_llvm::back::write::codegen::{{closure}} ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fd46cf15ff1 in rustc::util::common::time_ext () from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007fd46d0366c1 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::write::WriteBackendMethods>::codegen ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007fd46cf477c4 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007fd46cf4c5d6 in std::panicking::try::do_call () from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#19 0x00007fd4708cef5a in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:86
#20 0x00007fd46cf4ce16 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
   from /home/op/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#21 0x00007fd47089fcbf in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/5c5b8afd80e6fa1d24632153cb2257c686041d41/src/liballoc/boxed.rs:942
#22 0x00007fd4708cd980 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/5c5b8afd80e6fa1d24632153cb2257c686041d41/src/liballoc/boxed.rs:942
#23 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#24 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#25 0x00007fd47080d669 in start_thread (arg=<optimized out>) at pthread_create.c:479
#26 0x00007fd470722323 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
