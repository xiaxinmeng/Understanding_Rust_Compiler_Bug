
#5009 0x00007fffe5f3f1f7 in (anonymous namespace)::CopyRewriter::getNewSource(llvm::MachineRegisterInfo*, llvm::TargetInstrInfo const*, llvm::TargetInstrInfo::RegSubRegPair, llvm::SmallDenseMap<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult, 4u, llvm::DenseMapInfo<llvm::TargetInstrInfo::RegSubRegPair>, llvm::detail::DenseMapPair<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult> >&, bool) () from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5010 0x00007fffe5f3f1f7 in (anonymous namespace)::CopyRewriter::getNewSource(llvm::MachineRegisterInfo*, llvm::TargetInstrInfo const*, llvm::TargetInstrInfo::RegSubRegPair, llvm::SmallDenseMap<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult, 4u, llvm::DenseMapInfo<llvm::TargetInstrInfo::RegSubRegPair>, llvm::detail::DenseMapPair<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult> >&, bool) () from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5011 0x00007fffe5f3f1f7 in (anonymous namespace)::CopyRewriter::getNewSource(llvm::MachineRegisterInfo*, llvm::TargetInstrInfo const*, llvm::TargetInstrInfo::RegSubRegPair, llvm::SmallDenseMap<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult, 4u, llvm::DenseMapInfo<llvm::TargetInstrInfo::RegSubRegPair>, llvm::detail::DenseMapPair<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult> >&, bool) () from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5012 0x00007fffe5f3f1f7 in (anonymous namespace)::CopyRewriter::getNewSource(llvm::MachineRegisterInfo*, llvm::TargetInstrInfo const*, llvm::TargetInstrInfo::RegSubRegPair, llvm::SmallDenseMap<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult, 4u, llvm::DenseMapInfo<llvm::TargetInstrInfo::RegSubRegPair>, llvm::detail::DenseMapPair<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult> >&, bool) () from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5013 0x00007fffe5f3f1f7 in (anonymous namespace)::CopyRewriter::getNewSource(llvm::MachineRegisterInfo*, llvm::TargetInstrInfo const*, llvm::TargetInstrInfo::RegSubRegPair, llvm::SmallDenseMap<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult, 4u, llvm::DenseMapInfo<llvm::TargetInstrInfo::RegSubRegPair>, llvm::detail::DenseMapPair<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult> >&, bool) () from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5014 0x00007fffe5f3f72b in (anonymous namespace)::UncoalescableRewriter::RewriteSource(llvm::TargetInstrInfo::RegSubRegPair, llvm::SmallDenseMap<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult, 4u, llvm::DenseMapInfo<llvm::TargetInstrInfo::RegSubRegPair>, llvm::detail::DenseMapPair<llvm::TargetInstrInfo::RegSubRegPair, (anonymous namespace)::ValueTrackerResult> >&) ()
   from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5015 0x00007fffe5f419b1 in (anonymous namespace)::PeepholeOptimizer::runOnMachineFunction(llvm::MachineFunction&) [clone .part.206] ()
---Type <return> to continue, or q <return> to quit---
   from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5016 0x00007fffe5e8ce61 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5017 0x00007fffe685c851 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
   from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5018 0x00007fffe685c891 in llvm::FPPassManager::runOnModule(llvm::Module&) ()
   from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5019 0x00007fffe685c0a4 in llvm::legacy::PassManagerImpl::run(llvm::Module&) ()
   from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5020 0x00007fffe4ddf12b in LLVMRustWriteOutputFile () from rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_trans-llvm.so
#5021 0x00007fffe4d890a6 in rustc_trans::back::write::write_output_file::h741b06e231beddc6 (handler=0x7fffdd9ecc60, target=0x7fffbc000020, pm=0x28, m=0x28, output=0x7fffbc908600, 
    file_type=rustc_llvm::ffi::FileType::ObjectFile) at librustc_trans/back/write.rs:103
#5022 0x00007fffe4c8e55d in rustc_trans::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h9d98aef41bc041c6 (cpm=0x7fffbca593e0) at librustc_trans/back/write.rs:784
#5023 rustc_trans::back::write::codegen::with_codegen::hf90ccf009218e3f5 (tm=0x7fffbc000990, llmod=<optimized out>, f=..., no_builtins=<optimized out>) at librustc_trans/back/write.rs:661
#5024 rustc_trans::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::hbccad4da7b9f1c39 () at librustc_trans/back/write.rs:783
#5025 0x00007fffe4c7c924 in rustc::util::common::time_ext::hd2df3fc6208c6b1d (do_it=<optimized out>, sess=..., what=..., f=...) at rust/src/librustc/util/common.rs:139
#5026 0x00007fffe4d8b427 in rustc_trans::back::write::codegen::hb3f0e3d677842d4d (cgcx=<optimized out>, diag_handler=<optimized out>, mtrans=..., config=<optimized out>, timeline=<optimized out>)
    at librustc_trans/back/write.rs:717
#5027 0x00007fffe4d90f37 in rustc_trans::back::write::execute_work_item::h1d69e772b9c812fb (cgcx=0x7fffdd9ed270, work_item=..., timeline=0x7fffdd9ed580) at librustc_trans/back/write.rs:1250
#5028 0x00007fffe4c61990 in rustc_trans::back::write::spawn_work::_$u7b$$u7b$closure$u7d$$u7d$::h3437fe7e8588ce90 () at librustc_trans/back/write.rs:2008
#5029 std::sys_common::backtrace::__rust_begin_short_backtrace::he23cedfddbf530ac (f=...) at rust/src/libstd/sys_common/backtrace.rs:136
#5030 0x00007fffe4d081ee in std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h0ce6f254d07581a2 () at rust/src/libstd/thread/mod.rs:406
#5031 _$LT$std..panic..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h11cf8c96bef2521a (self=..., _args=<optimized out>)
    at rust/src/libstd/panic.rs:296
#5032 std::panicking::try::do_call::h717b090c00725633 (data=<optimized out>) at rust/src/libstd/panicking.rs:306
#5033 0x00007ffff76f9cff in __rust_maybe_catch_panic (f=0x28, data=0x7fffbc000020 "\001", data_ptr=0x7fffdd9edc20, vtable_ptr=0x7fffdd9edc18) at libpanic_unwind/lib.rs:102
#5034 0x00007fffe4d17a26 in std::panicking::try::hdb7dd180952ed9f4 (f=...) at rust/src/libstd/panicking.rs:285
#5035 std::panic::catch_unwind::h07619e3a50275ab1 (f=...) at rust/src/libstd/panic.rs:361
#5036 std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::hb12a476bc3f15d3c () at rust/src/libstd/thread/mod.rs:405
#5037 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h1150c29ad090aec6 (self=0x7fffd5312340, args=<optimized out>) at rust/src/liballoc/boxed.rs:783
#5038 0x00007ffff76cc0c8 in _$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h080a22996b1e6007
    (self=..., args=<optimized out>) at rust/src/liballoc/boxed.rs:793
#5039 std::sys_common::thread::start_thread::h003518bbe5f1d1a5 (main=0x7fffd52dec40 "@#1\325\377\177") at libstd/sys_common/thread.rs:24
#5040 0x00007ffff76de869 in std::sys::unix::thread::Thread::new::thread_start::h796b86e2ef2f99b8 (main=0x7fffbc000020) at libstd/sys/unix/thread.rs:90
#5041 0x00007ffff1a12519 in start_thread (arg=0x7fffdd9ee700) at pthread_create.c:456
#5042 0x00007ffff73a8a5f in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:97
