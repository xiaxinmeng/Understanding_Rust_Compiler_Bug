
(gdb) bt
#0  0x00007ff7c98b4d04 in llvm::DwarfCompileUnit::constructVariableDIEImpl(llvm::DbgVariable const&, bool) ()
   from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#1  0x00007ff7c98b1a7e in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) ()
   from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2  0x00007ff7c98b1696 in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) ()
   from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#3  0x00007ff7c98b1696 in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) ()
...
#2885 0x00007ff7c98b1696 in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2886 0x00007ff7c98b1696 in llvm::DwarfCompileUnit::createAndAddScopeChildren(llvm::LexicalScope*, llvm::DIE&) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2887 0x00007ff7c9ae823c in llvm::DwarfCompileUnit::constructSubprogramScopeDIE(llvm::DISubprogram const*, llvm::LexicalScope*) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2888 0x00007ff7c9ae7b2a in llvm::DwarfDebug::endFunctionImpl(llvm::MachineFunction const*) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2889 0x00007ff7c9add665 in llvm::DebugHandlerBase::endFunction(llvm::MachineFunction const*) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2890 0x00007ff7c95a0f60 in llvm::AsmPrinter::emitFunctionBody() () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2891 0x00007ff7c959e8d0 in llvm::X86AsmPrinter::runOnMachineFunction(llvm::MachineFunction&) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2892 0x00007ff7c9a3f704 in llvm::FPPassManager::runOnFunction(llvm::Function&) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2893 0x00007ff7c9a3edcf in llvm::FPPassManager::runOnModule(llvm::Module&) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2894 0x00007ff7c9a3e708 in llvm::legacy::PassManagerImpl::run(llvm::Module&) () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libLLVM-14-rust-1.64.0-stable.so
#2895 0x00007ff7ce37efd5 in LLVMRustWriteOutputFile () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-68b636f4229aa517.so
#2896 0x00007ff7ce37e914 in rustc_codegen_llvm::back::write::write_output_file () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-68b636f4229aa517.so
#2897 0x00007ff7ce37c95a in rustc_codegen_llvm::back::write::codegen () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-68b636f4229aa517.so
#2898 0x00007ff7ce378c68 in rustc_codegen_ssa::back::write::finish_intra_module_work::<rustc_codegen_llvm::LlvmCodegenBackend> () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-68b636f4229aa517.so
#2899 0x00007ff7ce377ed1 in rustc_codegen_ssa::back::write::execute_work_item::<rustc_codegen_llvm::LlvmCodegenBackend> () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-68b636f4229aa517.so
#2900 0x00007ff7ce37687e in std::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::spawn_named_thread<rustc_codegen_ssa::back::write::spawn_work<rustc_codegen_llvm::LlvmCodegenBackend>::{closure#0}, ()>::{closure#0}, ()> () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-68b636f4229aa517.so
#2901 0x00007ff7ce2f3687 in <<std::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::spawn_named_thread<rustc_codegen_ssa::back::write::spawn_work<rustc_codegen_llvm::LlvmCodegenBackend>::{closure#0}, ()>::{closure#0}, ()>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} () from /home/mexus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/librustc_driver-68b636f4229aa517.so
#2902 0x00007ff7cbf7a723 in alloc::boxed::{impl#44}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:1935
#2903 alloc::boxed::{impl#44}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:1935
#2904 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
#2905 0x00007ff7cbd1c8fd in start_thread (arg=<optimized out>) at pthread_create.c:442
#2906 0x00007ff7cbd9ea60 in clone3 () at ../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
