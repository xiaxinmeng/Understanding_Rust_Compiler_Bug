
* thread #1, name = 'rustc', stop reason = signal SIGSTOP
  * frame #0: 0x00007f94a22f3f6d libpthread.so.0`__GI___pthread_timedjoin_ex + 381
    frame #1: 0x00007f94a23c431d libstd-61dc3193374c0a18.so`join at thread.rs:166:22
    frame #2: 0x00007f94a2688793 librustc_driver-c22b57af01f99f73.so`std::thread::JoinHandle$LT$T$GT$::join::h8bf99f6b6378f358 + 67
    frame #3: 0x00007f94a2687e2e librustc_driver-c22b57af01f99f73.so`rustc_interface::util::spawn_thread_pool::hfbd6990d01deeee7 + 750
    frame #4: 0x00007f94a26f1a01 librustc_driver-c22b57af01f99f73.so`rustc_driver::run_compiler::hc77270b84c49fc37 + 6401
    frame #5: 0x00007f94a26f9ca3 librustc_driver-c22b57af01f99f73.so`_$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::hdf6334407d6a84a5 + 115
    frame #6: 0x00007f94a26a3409 librustc_driver-c22b57af01f99f73.so`std::panicking::try::do_call::h032b698e8fd89aa2 (.llvm.5291137794841497670) + 9
    frame #7: 0x00007f94a23c55ea libstd-61dc3193374c0a18.so`__rust_maybe_catch_panic at lib.rs:85:7
    frame #8: 0x00007f94a26f8222 librustc_driver-c22b57af01f99f73.so`rustc_driver::report_ices_to_stderr_if_any::he18c2a4bfc04bf18 + 66
    frame #9: 0x00007f94a26f8a8c librustc_driver-c22b57af01f99f73.so`rustc_driver::main::ha2a1da63c20faf7b + 12
    frame #10: 0x0000560d49426423 rustc`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h80c826ec4a9681b0 + 3
    frame #11: 0x00007f94a23b4133 libstd-61dc3193374c0a18.so`do_call<closure,i32> [inlined] {{closure}} at rt.rs:49:12
    frame #12: 0x00007f94a23b4127 libstd-61dc3193374c0a18.so`do_call<closure,i32> at panicking.rs:293
    frame #13: 0x00007f94a23c55ea libstd-61dc3193374c0a18.so`__rust_maybe_catch_panic at lib.rs:85:7
    frame #14: 0x00007f94a23b4cfd libstd-61dc3193374c0a18.so`lang_start_internal [inlined] try<i32,closure> at panicking.rs:272:12
    frame #15: 0x00007f94a23b4cbf libstd-61dc3193374c0a18.so`lang_start_internal [inlined] catch_unwind<closure,i32> at panic.rs:388
    frame #16: 0x00007f94a23b4cbf libstd-61dc3193374c0a18.so`lang_start_internal at rt.rs:48
    frame #17: 0x0000560d49426482 rustc`main + 34
    frame #18: 0x00007f94a213c223 libc.so.6`__libc_start_main + 243
    frame #19: 0x0000560d494262f9 rustc`_start + 41
    frame #20: 0x0000560d494262f9 rustc`_start + 41
  thread #2, name = 'rustc', stop reason = signal SIGSTOP
    frame #0: 0x00007f94a22f8afc libpthread.so.0`__pthread_cond_wait + 508
    frame #1: 0x00007f94a23974c3 libstd-61dc3193374c0a18.so`park [inlined] wait at condvar.rs:69:16
    frame #2: 0x00007f94a23974c1 libstd-61dc3193374c0a18.so`park [inlined] wait at condvar.rs:41
    frame #3: 0x00007f94a23974c1 libstd-61dc3193374c0a18.so`park [inlined] wait<()> at condvar.rs:204
    frame #4: 0x00007f94a23974a7 libstd-61dc3193374c0a18.so`park at mod.rs:910
    frame #5: 0x00007f94a23ad912 libstd-61dc3193374c0a18.so`wait at blocking.rs:71:12
    frame #6: 0x00007f94980d3c6f librustc_codegen_ssa-6ba4787921c69f6c.so`std::sync::mpsc::shared::Packet$LT$T$GT$::recv::h2247219f7a4b2a23 + 687
    frame #7: 0x00007f949813e6e1 librustc_codegen_ssa-6ba4787921c69f6c.so`std::sync::mpsc::Receiver$LT$T$GT$::recv::hd575e115e4966bdd + 961
    frame #8: 0x00007f9498147f65 librustc_codegen_ssa-6ba4787921c69f6c.so`rustc_codegen_ssa::back::write::SharedEmitterMain::check::hbebe119c0a152ed8 + 101
    frame #9: 0x00007f94985bfc27 librustc_codegen_llvm-llvm.so`_$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_utils..codegen_backend..CodegenBackend$GT$::join_codegen_and_link::h6e439f1f52aa7a33 + 135
    frame #10: 0x00007f94a1d62e0d librustc_interface-c50f073a346131db.so`rustc_interface::queries::Query$LT$T$GT$::compute::h5ace5e729a26801a + 541
    frame #11: 0x00007f94a1e34ce4 librustc_interface-c50f073a346131db.so`rustc_interface::queries::_$LT$impl$u20$rustc_interface..interface..Compiler$GT$::link::hbfa5a2fb5b8e97e8 + 20
    frame #12: 0x00007f94a2698b17 librustc_driver-c22b57af01f99f73.so`rustc_interface::interface::run_compiler_in_existing_thread_pool::h80e01640c1bd2550 + 3959
    frame #13: 0x00007f94a266d816 librustc_driver-c22b57af01f99f73.so`std::thread::local::LocalKey$LT$T$GT$::with::h1d29af37c0300c7d + 294
    frame #14: 0x00007f94a26d01f5 librustc_driver-c22b57af01f99f73.so`scoped_tls::ScopedKey$LT$T$GT$::set::hcd31602a4fdbc7a8 + 533
    frame #15: 0x00007f94a2704902 librustc_driver-c22b57af01f99f73.so`syntax::with_globals::h6f9bb4d03b81c38c + 82
    frame #16: 0x00007f94a266f551 librustc_driver-c22b57af01f99f73.so`std::sys_common::backtrace::__rust_begin_short_backtrace::hd75831c1c8cc1ae1 + 369
    frame #17: 0x00007f94a23c55ea libstd-61dc3193374c0a18.so`__rust_maybe_catch_panic at lib.rs:85:7
    frame #18: 0x00007f94a26896b9 librustc_driver-c22b57af01f99f73.so`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hdec1144d73d6cc23 + 121
    frame #19: 0x00007f94a23966ff libstd-61dc3193374c0a18.so`call_once<(),FnOnce<()>> at boxed.rs:704:8
    frame #20: 0x00007f94a23c4260 libstd-61dc3193374c0a18.so`thread_start [inlined] call_once<(),alloc::boxed::Box<FnOnce<()>>> at boxed.rs:704:8
    frame #21: 0x00007f94a23c4254 libstd-61dc3193374c0a18.so`thread_start [inlined] start_thread at thread.rs:13
    frame #22: 0x00007f94a23c41da libstd-61dc3193374c0a18.so`thread_start at thread.rs:79
    frame #23: 0x00007f94a22f2a9d libpthread.so.0`start_thread + 253
    frame #24: 0x00007f94a2213af3 libc.so.6`__GI___clone + 67
  thread #3, name = 'rustc', stop reason = signal SIGSTOP
    frame #0: 0x00007f94a22f8afc libpthread.so.0`__pthread_cond_wait + 508
    frame #1: 0x00007f94a23974c3 libstd-61dc3193374c0a18.so`park [inlined] wait at condvar.rs:69:16
    frame #2: 0x00007f94a23974c1 libstd-61dc3193374c0a18.so`park [inlined] wait at condvar.rs:41
    frame #3: 0x00007f94a23974c1 libstd-61dc3193374c0a18.so`park [inlined] wait<()> at condvar.rs:204
    frame #4: 0x00007f94a23974a7 libstd-61dc3193374c0a18.so`park at mod.rs:910
    frame #5: 0x00007f94a23ad912 libstd-61dc3193374c0a18.so`wait at blocking.rs:71:12
    frame #6: 0x00007f949e8cc866 librustc_data_structures-ed17cfdcb80318ba.so`std::sync::mpsc::stream::Packet$LT$T$GT$::recv::hc50c85645d74ade0 + 518
    frame #7: 0x00007f949e8d2ad9 librustc_data_structures-ed17cfdcb80318ba.so`std::sync::mpsc::Receiver$LT$T$GT$::recv::h0569fcd7a70d11bc (.llvm.14781405754736769625) + 217
    frame #8: 0x00007f949e8cf11a librustc_data_structures-ed17cfdcb80318ba.so`std::sys_common::backtrace::__rust_begin_short_backtrace::h16a86ed6e74ac76f + 106
    frame #9: 0x00007f949e8cdc4e librustc_data_structures-ed17cfdcb80318ba.so`std::panicking::try::do_call::h72a4f6ce322dbcdb (.llvm.16852748950644158492) + 46
    frame #10: 0x00007f94a23c55ea libstd-61dc3193374c0a18.so`__rust_maybe_catch_panic at lib.rs:85:7
    frame #11: 0x00007f949e8ce344 librustc_data_structures-ed17cfdcb80318ba.so`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::ha026c63fac5de9fe + 148
    frame #12: 0x00007f94a23966ff libstd-61dc3193374c0a18.so`call_once<(),FnOnce<()>> at boxed.rs:704:8
    frame #13: 0x00007f94a23c4260 libstd-61dc3193374c0a18.so`thread_start [inlined] call_once<(),alloc::boxed::Box<FnOnce<()>>> at boxed.rs:704:8
    frame #14: 0x00007f94a23c4254 libstd-61dc3193374c0a18.so`thread_start [inlined] start_thread at thread.rs:13
    frame #15: 0x00007f94a23c41da libstd-61dc3193374c0a18.so`thread_start at thread.rs:79
    frame #16: 0x00007f94a22f2a9d libpthread.so.0`start_thread + 253
    frame #17: 0x00007f94a2213af3 libc.so.6`__GI___clone + 67
  thread #4, name = 'rustc', stop reason = signal SIGSTOP
    frame #0: 0x00007f94a22f8afc libpthread.so.0`__pthread_cond_wait + 508
    frame #1: 0x00007f94a23974c3 libstd-61dc3193374c0a18.so`park [inlined] wait at condvar.rs:69:16
    frame #2: 0x00007f94a23974c1 libstd-61dc3193374c0a18.so`park [inlined] wait at condvar.rs:41
    frame #3: 0x00007f94a23974c1 libstd-61dc3193374c0a18.so`park [inlined] wait<()> at condvar.rs:204
    frame #4: 0x00007f94a23974a7 libstd-61dc3193374c0a18.so`park at mod.rs:910
    frame #5: 0x00007f94a23ad912 libstd-61dc3193374c0a18.so`wait at blocking.rs:71:12
    frame #6: 0x00007f94986348f6 librustc_codegen_llvm-llvm.so`std::sync::mpsc::shared::Packet$LT$T$GT$::recv::h79e1e1d936784cae + 614
    frame #7: 0x00007f94985a0a1a librustc_codegen_llvm-llvm.so`std::sync::mpsc::Receiver$LT$T$GT$::recv::h19f647c010a0be90 + 762
    frame #8: 0x00007f949852e156 librustc_codegen_llvm-llvm.so`std::sys_common::backtrace::__rust_begin_short_backtrace::hb64d0b815da7bda5 + 6342
    frame #9: 0x00007f949855bd1e librustc_codegen_llvm-llvm.so`std::panicking::try::do_call::h9d5587ac7428c57e (.llvm.6676359552974513345) + 62
    frame #10: 0x00007f94a23c55ea libstd-61dc3193374c0a18.so`__rust_maybe_catch_panic at lib.rs:85:7
    frame #11: 0x00007f9498532567 librustc_codegen_llvm-llvm.so`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::ha73c43c769e33e4a + 135
    frame #12: 0x00007f94a23966ff libstd-61dc3193374c0a18.so`call_once<(),FnOnce<()>> at boxed.rs:704:8
    frame #13: 0x00007f94a23c4260 libstd-61dc3193374c0a18.so`thread_start [inlined] call_once<(),alloc::boxed::Box<FnOnce<()>>> at boxed.rs:704:8
    frame #14: 0x00007f94a23c4254 libstd-61dc3193374c0a18.so`thread_start [inlined] start_thread at thread.rs:13
    frame #15: 0x00007f94a23c41da libstd-61dc3193374c0a18.so`thread_start at thread.rs:79
    frame #16: 0x00007f94a22f2a9d libpthread.so.0`start_thread + 253
    frame #17: 0x00007f94a2213af3 libc.so.6`__GI___clone + 67
  thread #5, name = 'rustc', stop reason = signal SIGSTOP
    frame #0: 0x00007f94959a2ff6 libLLVM-8-rust-1.36.0-nightly.so`llvm::ValueHandleBase::RemoveFromUseList() + 118
    frame #1: 0x00007f949630d60c libLLVM-8-rust-1.36.0-nightly.so`llvm::InstCombiner::visitAllocSite(llvm::Instruction&) + 3484
    frame #2: 0x00007f949634b58a libLLVM-8-rust-1.36.0-nightly.so`llvm::InstCombiner::visitCallInst(llvm::CallInst&) + 410
    frame #3: 0x00007f94963142e0 libLLVM-8-rust-1.36.0-nightly.so`combineInstructionsOverFunction(llvm::Function&, llvm::InstCombineWorklist&, llvm::AAResults*, llvm::AssumptionCache&, llvm::TargetLibraryInfo&, llvm::DominatorTree&, llvm::OptimizationRemarkEmitter&, bool, llvm::LoopInfo*) + 7792
    frame #4: 0x00007f9496314f57 libLLVM-8-rust-1.36.0-nightly.so`llvm::InstructionCombiningPass::runOnFunction(llvm::Function&) + 695
    frame #5: 0x00007f949595493a libLLVM-8-rust-1.36.0-nightly.so`llvm::FPPassManager::runOnFunction(llvm::Function&) + 2634
    frame #6: 0x00007f949683d068 libLLVM-8-rust-1.36.0-nightly.so`(anonymous namespace)::CGPassManager::runOnModule(llvm::Module&) + 1400
    frame #7: 0x00007f94959554a2 libLLVM-8-rust-1.36.0-nightly.so`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 1762
    frame #8: 0x00007f94958bfe5a libLLVM-8-rust-1.36.0-nightly.so`LLVMRunPassManager + 10
    frame #9: 0x00007f9498646056 librustc_codegen_llvm-llvm.so`rustc_codegen_llvm::back::write::optimize::h69768f3f42181523 + 2102
    frame #10: 0x00007f9498596849 librustc_codegen_llvm-llvm.so`rustc_codegen_ssa::back::write::execute_work_item::h5c92abdccfeba56b + 3081
    frame #11: 0x00007f949852c626 librustc_codegen_llvm-llvm.so`std::sys_common::backtrace::__rust_begin_short_backtrace::h079fc7ed9043c730 + 246
    frame #12: 0x00007f949855bd66 librustc_codegen_llvm-llvm.so`std::panicking::try::do_call::hb87ab877f663bdea (.llvm.6676359552974513345) + 38
    frame #13: 0x00007f94a23c55ea libstd-61dc3193374c0a18.so`__rust_maybe_catch_panic at lib.rs:85:7
    frame #14: 0x00007f9498532386 librustc_codegen_llvm-llvm.so`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h55415b3cd65d838d + 134
    frame #15: 0x00007f94a23966ff libstd-61dc3193374c0a18.so`call_once<(),FnOnce<()>> at boxed.rs:704:8
    frame #16: 0x00007f94a23c4260 libstd-61dc3193374c0a18.so`thread_start [inlined] call_once<(),alloc::boxed::Box<FnOnce<()>>> at boxed.rs:704:8
    frame #17: 0x00007f94a23c4254 libstd-61dc3193374c0a18.so`thread_start [inlined] start_thread at thread.rs:13
    frame #18: 0x00007f94a23c41da libstd-61dc3193374c0a18.so`thread_start at thread.rs:79
    frame #19: 0x00007f94a22f2a9d libpthread.so.0`start_thread + 253
    frame #20: 0x00007f94a2213af3 libc.so.6`__GI___clone + 67
