
#0  0x00007fffef80b596 in llvm::LiveVariables::HandleVirtRegUse(unsigned int, llvm::MachineBasicBlock*, llvm::MachineInstr&) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fffef80d73e in llvm::LiveVariables::runOnInstr(llvm::MachineInstr&, llvm::SmallVectorImpl<unsigned int>&) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fffef80db2f in llvm::LiveVariables::runOnBlock(llvm::MachineBasicBlock*, unsigned int) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fffef80e443 in llvm::LiveVariables::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fffef86369a in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007ffff030962e in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007ffff03099d3 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007ffff030a0f4 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fffee3c149c in LLVMRustWriteOutputFile ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fffee22064a in rustc_codegen_llvm::back::write::write_output_file ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fffee1f5b15 in rustc_codegen_llvm::back::write::codegen::{{closure}} ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fffee1ed168 in rustc::util::common::time_ext ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fffee222b9a in rustc_codegen_llvm::back::write::codegen ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fffee228fcd in rustc_codegen_llvm::back::write::execute_work_item ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fffee2026ed in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x00007fffee219bc6 in std::panicking::try::do_call ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007ffff78c57da in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#17 0x00007fffee20b371 in <F as alloc::boxed::FnBox<A>>::call_box ()
   from /home/lkurusa/.rustup/toolchains/nightly-2018-09-09-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#18 0x00007ffff78b33eb in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hf8f28b18ff71f43a () at /checkout/src/liballoc/boxed.rs:656
#19 std::sys_common::thread::start_thread () at libstd/sys_common/thread.rs:24
#20 0x00007ffff7886e86 in std::sys::unix::thread::Thread::new::thread_start () at libstd/sys/unix/thread.rs:90
#21 0x00007ffff207ba9d in start_thread () from /usr/lib/libpthread.so.0
#22 0x00007ffff7743a43 in clone () from /usr/lib/libc.so.6
