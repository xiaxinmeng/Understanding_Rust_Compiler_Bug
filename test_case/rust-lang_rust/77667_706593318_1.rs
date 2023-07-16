
#0  0x00007ffb5521a615 in raise () from /usr/lib/libc.so.6
#1  0x00007ffb55203862 in abort () from /usr/lib/libc.so.6
#2  0x00007ffb52ee49a1 in llvm::llvm_unreachable_internal(char const*, char const*, unsigned int) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#3  0x00007ffb54920f56 in llvm::ArrayRef<unsigned char> llvm::codeview::SimpleTypeSerializer::serialize<llvm::codeview::ClassRecord>(llvm::codeview::ClassRecord&) ()
   from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#4  0x00007ffb5392f190 in llvm::CodeViewDebug::lowerTypeClass(llvm::DICompositeType const*) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#5  0x00007ffb53924cec in llvm::CodeViewDebug::getTypeIndex(llvm::DIType const*, llvm::DIType const*) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#6  0x00007ffb5392e78c in llvm::CodeViewDebug::lowerTypeFunction(llvm::DISubroutineType const*) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#7  0x00007ffb53924cec in llvm::CodeViewDebug::getTypeIndex(llvm::DIType const*, llvm::DIType const*) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#8  0x00007ffb53924149 in llvm::CodeViewDebug::getFuncIdForSubprogram(llvm::DISubprogram const*) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#9  0x00007ffb53927c57 in llvm::CodeViewDebug::emitDebugInfoForFunction(llvm::Function const*, llvm::CodeViewDebug::FunctionInfo&) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#10 0x00007ffb5392627f in llvm::CodeViewDebug::endModule() () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#11 0x00007ffb538b942d in llvm::AsmPrinter::doFinalization(llvm::Module&) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#12 0x00007ffb53123765 in llvm::FPPassManager::doFinalization(llvm::Module&) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#13 0x00007ffb5311d4a6 in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/aaron/repos/rust/weird/bin/../lib/../lib/libLLVM-11-rust-dev.so
#14 0x00007ffb56143635 in LLVMRustWriteOutputFile () from /home/aaron/repos/rust/weird/bin/../lib/librustc_driver-791e4e962741912d.so
#15 0x00007ffb55f7a3c8 in rustc_codegen_llvm::back::write::write_output_file () at compiler/rustc_codegen_llvm/src/back/write.rs:56
#16 0x00007ffb56099cf7 in rustc_codegen_llvm::back::write::codegen::{{closure}} () at compiler/rustc_codegen_llvm/src/back/write.rs:803
#17 rustc_codegen_llvm::back::write::codegen::with_codegen () at compiler/rustc_codegen_llvm/src/back/write.rs:691
#18 0x00007ffb55f8c88e in rustc_codegen_llvm::back::write::codegen () at compiler/rustc_codegen_llvm/src/back/write.rs:802
#19 <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::write::WriteBackendMethods>::codegen () at compiler/rustc_codegen_llvm/src/lib.rs:174
#20 0x00007ffb55f683bc in rustc_codegen_ssa::back::write::finish_intra_module_work () at /home/aaron/repos/rust/compiler/rustc_codegen_ssa/src/back/write.rs:887
#21 0x00007ffb55f624a7 in rustc_codegen_ssa::back::write::execute_optimize_work_item () at /home/aaron/repos/rust/compiler/rustc_codegen_ssa/src/back/write.rs:806
#22 rustc_codegen_ssa::back::write::execute_work_item () at /home/aaron/repos/rust/compiler/rustc_codegen_ssa/src/back/write.rs:722
#23 0x00007ffb560ca1de in rustc_codegen_ssa::back::write::spawn_work::{{closure}} () at /home/aaron/repos/rust/compiler/rustc_codegen_ssa/src/back/write.rs:1588
#24 std::sys_common::backtrace::__rust_begin_short_backtrace () at /home/aaron/repos/rust/library/std/src/sys_common/backtrace.rs:137
#25 0x00007ffb5609653d in std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}} () at /home/aaron/repos/rust/library/std/src/thread/mod.rs:464
#26 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at /home/aaron/repos/rust/library/std/src/panic.rs:308
#27 std::panicking::try::do_call () at /home/aaron/repos/rust/library/std/src/panicking.rs:381
#28 std::panicking::try () at /home/aaron/repos/rust/library/std/src/panicking.rs:345
#29 std::panic::catch_unwind () at /home/aaron/repos/rust/library/std/src/panic.rs:382
#30 std::thread::Builder::spawn_unchecked::{{closure}} () at /home/aaron/repos/rust/library/std/src/thread/mod.rs:463
#31 core::ops::function::FnOnce::call_once{{vtable-shim}} () at /home/aaron/repos/rust/library/core/src/ops/function.rs:227
#32 0x00007ffb55480e08 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /home/aaron/repos/rust/library/alloc/src/boxed.rs:1042
#33 <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /home/aaron/repos/rust/library/alloc/src/boxed.rs:1042
#34 0x00007ffb55489a9a in std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:87
#35 0x00007ffb515643e9 in start_thread () from /usr/lib/libpthread.so.0
#36 0x00007ffb552dd293 in clone () from /usr/lib/libc.so.6
