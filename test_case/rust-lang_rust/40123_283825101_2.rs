
#0  0x00007ffff736d7ef in raise () from /lib/x86_64-linux-gnu/libc.so.6
#1  0x00007ffff736f3ea in abort () from /lib/x86_64-linux-gnu/libc.so.6
#2  0x00007ffff7365bb7 in ?? () from /lib/x86_64-linux-gnu/libc.so.6
#3  0x00007ffff7365c62 in __assert_fail () from /lib/x86_64-linux-gnu/libc.so.6
#4  0x00007ffff15c300e in (anonymous namespace)::ARMConstantIslands::doInitialConstPlacement(std::vector<llvm::MachineInstr*, std::allocator<llvm::MachineInstr*> >&) [clone .constprop.328] () from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-7c760c0d87163cbb.so
#5  0x00007ffff15c8908 in (anonymous namespace)::ARMConstantIslands::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-7c760c0d87163cbb.so
#6  0x00007ffff1bb8ac5 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-7c760c0d87163cbb.so
#7  0x00007ffff2458c13 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-7c760c0d87163cbb.so
#8  0x00007ffff2458cdc in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-7c760c0d87163cbb.so
#9  0x00007ffff245877d in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-7c760c0d87163cbb.so
#10 0x00007ffff0d69903 in LLVMRustWriteOutputFile ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_llvm-7c760c0d87163cbb.so
#11 0x00007ffff606d834 in rustc_trans::back::write::write_output_file::h713482cdaa178321 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_trans-efa094249cef739c.so
#12 0x00007ffff606fd3f in rustc_trans::back::write::optimize_and_codegen::_$u7b$$u7b$closure$u7d$$u7d$::h1b757de20fba8b9a ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_trans-efa094249cef739c.so
#13 0x00007ffff6074c26 in rustc_trans::back::write::execute_work_item::hbc7cd39d8cd88c44 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_trans-efa094249cef739c.so
#14 0x00007ffff607167a in rustc_trans::back::write::run_passes::h7cf08f2c47729966 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_trans-efa094249cef739c.so
#15 0x00007ffff7b52353 in rustc_driver::driver::phase_5_run_llvm_passes::h7c9bbedc8969c4b3 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-b65378ba5ffb99d4.so
#16 0x00007ffff7b18aa4 in rustc_driver::driver::compile_input::hf3e3aa4173908b86 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-b65378ba5ffb99d4.so
#17 0x00007ffff7b646d6 in rustc_driver::run_compiler::h8f8d47f1d258a8a6 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-b65378ba5ffb99d4.so
#18 0x00007ffff7a84115 in std::panicking::try::do_call::h206b9daee04f4ea2 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-b65378ba5ffb99d4.so
#19 0x00007ffff77a8077 in __rust_maybe_catch_panic ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libstd-9a66b6a343d52844.so
#20 0x00007ffff7aa0e62 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h5d196fbb3229f499 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-b65378ba5ffb99d4.so
#21 0x00007ffff779d248 in std::sys::imp::thread::Thread::new::thread_start::h2c901daa88f3cb32 ()
   from /home/logic/build-tmp/rust-src/obj/arm-android/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libstd-9a66b6a343d52844.so
#22 0x00007fffeefda6ca in start_thread () from /lib/x86_64-linux-gnu/libpthread.so.0
#23 0x00007ffff74400af in clone () from /lib/x86_64-linux-gnu/libc.so.6
