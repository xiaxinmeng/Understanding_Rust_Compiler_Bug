rust
#0  0x00007fffeff32159 in llvm::APInt::rotl(unsigned int) const ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#1  0x00007fffeff3238b in llvm::APInt::isSplat(unsigned int) const ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#2  0x00007fffefc9d1c7 in llvm::isBytewiseValue(llvm::Value*) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#3  0x00007fffef8fe7d7 in llvm::MemCpyOptPass::processStore(llvm::StoreInst*, llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, false, false>&) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#4  0x00007fffef8fff2b in llvm::MemCpyOptPass::iterateOnFunction(llvm::Function&) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#5  0x00007fffef900239 in llvm::MemCpyOptPass::runImpl(llvm::Function&, llvm::MemoryDependenceResults*, llvm::TargetLibraryInfo*, std::function<llvm::AAResults& ()>, std::function<llvm::AssumptionCache& ()>, std::function<llvm::DominatorTree& ()>) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#6  0x00007fffef9003fc in (anonymous namespace)::MemCpyOptLegacyPass::runOnFunction(llvm::Function&) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#7  0x00007fffefdf5f4a in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#8  0x00007fffefb6c5df in (anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#9  0x00007fffefdf6950 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#10 0x00007fffefd8d879 in LLVMRunPassManager ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/../lib/librustc_llvm-50edb4a75bc0fe7c.so
#11 0x00007ffff5b592f5 in rustc::util::common::time ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/librustc_trans-64828bfa1a07d0dd.so
#12 0x00007ffff5b1eaba in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/librustc_trans-64828bfa1a07d0dd.so
#13 0x00007ffff5b23408 in std::panicking::try::do_call ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/librustc_trans-64828bfa1a07d0dd.so
#14 0x00007ffff76b3e6f in panic_unwind::__rust_maybe_catch_panic () at /checkout/src/libpanic_unwind/lib.rs:101
#15 0x00007ffff5b29d0b in <F as alloc::boxed::FnBox<A>>::call_box ()
   from /home/simon/tmp/servo-share/rust/2017-11-21/rustc-nightly-x86_64-unknown-linux-gnu/rustc/bin/../lib/../lib/librustc_trans-64828bfa1a07d0dd.so
#16 0x00007ffff76a73cc in alloc::boxed::{{impl}}::call_once<(),()> () at /checkout/src/liballoc/boxed.rs:782
#17 std::sys_common::thread::start_thread () at /checkout/src/libstd/sys_common/thread.rs:24
#18 std::sys::imp::thread::{{impl}}::new::thread_start () at /checkout/src/libstd/sys/unix/thread.rs:90
#19 0x00007ffff15e27fc in start_thread (arg=0x7fff661ff700) at pthread_create.c:465
#20 0x00007ffff7362b0f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95