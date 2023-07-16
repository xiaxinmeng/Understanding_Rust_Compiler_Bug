
(gdb) bt
#0  0x00007fffeee66962 in void llvm::VerifierSupport::WriteTs<llvm::Function*, llvm::Module*>(llvm::Function* const&, llvm::Module* const&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#1  0x00007fffeee70d93 in (anonymous namespace)::Verifier::visitInstruction(llvm::Instruction&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#2  0x00007fffeee7d93c in (anonymous namespace)::Verifier::visitCallInst(llvm::CallInst&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#3  0x00007fffeee584e8 in (anonymous namespace)::Verifier::verify(llvm::Function const&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#4  0x00007fffeee7eff3 in (anonymous namespace)::VerifierLegacyPass::runOnFunction(llvm::Function&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#5  0x00007fffeee0242f in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#6  0x00007fffeee02893 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#7  0x00007fffeee03030 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/../lib/libLLVM-9-rust-1.40.0-nightly.so
#8  0x00007ffff18791bf in LLVMRustWriteOutputFile ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007ffff182478c in rustc_codegen_llvm::back::write::write_output_file ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007ffff17179e6 in rustc_codegen_llvm::back::write::codegen::{{closure}} ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007ffff17139f1 in rustc::util::common::time_ext ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007ffff1834221 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::write::WriteBackendMethods>::codegen ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007ffff174a294 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007ffff174bcb6 in std::panicking::try::do_call ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007ffff511eeea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:79
#16 0x00007ffff1726706 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
   from /home/jistone/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x00007ffff50efc8f in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/3fc30d884ae0c988d98452a06737705cfe34806a/src/liballoc/boxed.rs:942
#18 0x00007ffff511d910 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/3fc30d884ae0c988d98452a06737705cfe34806a/src/liballoc/boxed.rs:942
#19 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#20 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#21 0x00007ffff505d4e2 in start_thread () from /lib64/libpthread.so.0
#22 0x00007ffff4f7a643 in clone () from /lib64/libc.so.6
