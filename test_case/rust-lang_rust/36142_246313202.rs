
#0  0x00007ffff731904f in raise () from /usr/lib/libc.so.6
#1  0x00007ffff731a47a in abort () from /usr/lib/libc.so.6
#2  0x00007ffff293fd75 in llvm::llvm_unreachable_internal(char const*, char const*, unsigned int) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#3  0x00007ffff193e45d in llvm::ARMTargetLowering::getEffectiveCallingConv(unsigned int, bool) const () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#4  0x00007ffff193e5ab in llvm::ARMTargetLowering::CCAssignFnForNode(unsigned int, bool, bool) const () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#5  0x00007ffff193e6e7 in llvm::ARMTargetLowering::CanLowerReturn(unsigned int, llvm::MachineFunction&, bool, llvm::SmallVectorImpl<llvm::ISD::OutputArg> const&, llvm::LLVMContext&) const ()
   from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#6  0x00007ffff1c88b78 in llvm::FunctionLoweringInfo::set(llvm::Function const&, llvm::MachineFunction&, llvm::SelectionDAG*) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#7  0x00007ffff1ddf803 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#8  0x00007ffff19301c4 in (anonymous namespace)::ARMDAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#9  0x00007ffff1fce3eb in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#10 0x00007ffff28aaec7 in llvm::FPPassManager::runOnFunction(llvm::Function&) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#11 0x00007ffff28aaf0b in llvm::FPPassManager::runOnModule(llvm::Module&) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#12 0x00007ffff28acadf in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_llvm-411f48d3.so
#13 0x00007ffff12e4406 in LLVMRustWriteOutputFile (Target=0x7fffed5cbd00, PMR=0x7fffed41a360, M=0x7fffed459f80, path=0x7fffed43ff50 "test.0.s", rust_FileType=<optimized out>)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/rustllvm/PassWrapper.cpp:489
#14 0x00007ffff627a4a4 in rustc_trans::back::write::write_output_file::hc9d219d70162da2f () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_trans-411f48d3.so
#15 0x00007ffff632d6a3 in rustc_trans::back::write::optimize_and_codegen::_$u7b$$u7b$closure$u7d$$u7d$::h81be959d30d696f6 () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_trans-411f48d3.so
#16 0x00007ffff62820de in rustc_trans::back::write::execute_work_item::hb51afc6495293b14 () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_trans-411f48d3.so
#17 0x00007ffff627cb6a in rustc_trans::back::write::run_passes::h56d72398bc66cc68 () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_trans-411f48d3.so
#18 0x00007ffff7b3fc9a in rustc_driver::driver::phase_5_run_llvm_passes::hac0993c14b71e09d () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-411f48d3.so
#19 0x00007ffff7b28c7b in rustc_driver::driver::compile_input::h7dacd98cd2fd7d2b () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-411f48d3.so
#20 0x00007ffff7b54337 in rustc_driver::run_compiler::h37c4294ab73436f7 () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-411f48d3.so
#21 0x00007ffff7a92c34 in std::panicking::try::do_call::h4d040997e2efdaf3 () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-411f48d3.so
#22 0x00007ffff77908c7 in __rust_maybe_catch_panic () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-411f48d3.so
#23 0x00007ffff7ab04ca in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hefd1c85fab3e6c31 () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/librustc_driver-411f48d3.so
#24 0x00007ffff777f181 in std::sys::thread::Thread::new::thread_start::h5b631f48cd23f128 () from /home/amanieu/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libstd-411f48d3.so
#25 0x00007fffef86c454 in start_thread () from /usr/lib/libpthread.so.0
#26 0x00007ffff73ce7df in clone () from /usr/lib/libc.so.6
