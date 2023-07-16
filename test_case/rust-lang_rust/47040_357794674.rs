
Thread 44 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x7fffe63ff700 (LWP 4960)]
0x00007fffef451493 in (anonymous namespace)::AddressingModeMatcher::matchOperationAddr(llvm::User*, unsigned int, unsigned int, bool*) [clone .part.974] ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
(gdb) bt
#0  0x00007fffef451493 in (anonymous namespace)::AddressingModeMatcher::matchOperationAddr(llvm::User*, unsigned int, unsigned int, bool*) [clone .part.974] ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#1  0x00007fffef4518fa in (anonymous namespace)::AddressingModeMatcher::matchAddr(llvm::Value*, unsigned int) ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#2  0x00007fffef4558e9 in (anonymous namespace)::CodeGenPrepare::optimizeMemoryInst(llvm::Instruction*, llvm::Value*, llvm::Type*, unsigned int) ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#3  0x00007fffef456c75 in (anonymous namespace)::CodeGenPrepare::optimizeInst(llvm::Instruction*, bool&) ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#4  0x00007fffef45985f in (anonymous namespace)::CodeGenPrepare::runOnFunction(llvm::Function&) [clone .part.1001] ()
   from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#5  0x00007fffefc40a6a in llvm::FPPassManager::runOnFunction(llvm::Function&) () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#6  0x00007fffefc40b03 in llvm::FPPassManager::runOnModule(llvm::Module&) () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#7  0x00007fffefc41470 in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#8  0x00007fffee6b1927 in LLVMRustWriteOutputFile () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/librustc_llvm-6d0682d527efa802.so
#9  0x00007ffff6d3b906 in rustc_trans::back::write::write_output_file () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-e8e6d6e4056c58ff.so
#10 0x00007ffff6d3e8a0 in rustc_trans::back::write::codegen::{{closure}} () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-e8e6d6e4056c58ff.so
#11 0x00007ffff6d3d9e4 in rustc_trans::back::write::codegen () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-e8e6d6e4056c58ff.so
#12 0x00007ffff6d28d98 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-e8e6d6e4056c58ff.so
#13 0x00007ffff6d2bd38 in std::panicking::try::do_call () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-e8e6d6e4056c58ff.so
#14 0x00007ffff76a96ff in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:102
#15 0x00007ffff6d3665b in <F as alloc::boxed::FnBox<A>>::call_box () from /home/roughl/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_trans-e8e6d6e4056c58ff.so
#16 0x00007ffff767e6f8 in _$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hecde7ab682048b67 ()
    at /checkout/src/liballoc/boxed.rs:827
#17 std::sys_common::thread::start_thread () at libstd/sys_common/thread.rs:24
#18 0x00007ffff7687cc9 in std::sys::unix::thread::Thread::new::thread_start () at libstd/sys/unix/thread.rs:90
#19 0x00007ffff16ae08c in start_thread () from /usr/lib/libpthread.so.0
#20 0x00007ffff7351e1f in clone () from /usr/lib/libc.so.6
