
Thread 5 Crashed:
0   libsystem_kernel.dylib        	0x00007fff7275c47a __pthread_kill + 10
1   libsystem_pthread.dylib       	0x00007fff72819707 pthread_kill + 384
2   libsystem_c.dylib             	0x00007fff726e4a08 abort + 120
3   librustc_codegen_llvm-llvm.dylib	0x000000010585f41e llvm::llvm_unreachable_internal(char const*, char const*, unsigned int) + 462
4   librustc_codegen_llvm-llvm.dylib	0x0000000103d8553d llvm::X86AsmPrinter::PrintOperand(llvm::MachineInstr const*, unsigned int, llvm::raw_ostream&) + 493
5   librustc_codegen_llvm-llvm.dylib	0x0000000103d85f50 llvm::X86AsmPrinter::PrintIntelMemReference(llvm::MachineInstr const*, unsigned int, llvm::raw_ostream&) + 752
6   librustc_codegen_llvm-llvm.dylib	0x0000000103d86679 llvm::X86AsmPrinter::PrintAsmMemoryOperand(llvm::MachineInstr const*, unsigned int, char const*, llvm::raw_ostream&) + 57
7   librustc_codegen_llvm-llvm.dylib	0x00000001042bf513 llvm::AsmPrinter::EmitInlineAsm(llvm::MachineInstr const*) const + 1939
8   librustc_codegen_llvm-llvm.dylib	0x00000001042ada06 llvm::AsmPrinter::EmitFunctionBody() + 2854
9   librustc_codegen_llvm-llvm.dylib	0x0000000103d84848 llvm::X86AsmPrinter::runOnMachineFunction(llvm::MachineFunction&) + 296
10  librustc_codegen_llvm-llvm.dylib	0x00000001044c7c6a llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 330
11  librustc_codegen_llvm-llvm.dylib	0x000000010518ddf0 llvm::FPPassManager::runOnFunction(llvm::Function&) + 1040
12  librustc_codegen_llvm-llvm.dylib	0x000000010518e113 llvm::FPPassManager::runOnModule(llvm::Module&) + 131
13  librustc_codegen_llvm-llvm.dylib	0x000000010518e5a2 llvm::legacy::PassManagerImpl::run(llvm::Module&) + 898
14  librustc_codegen_llvm-llvm.dylib	0x0000000103d5f41f LLVMRustWriteOutputFile + 559
15  librustc_codegen_llvm-llvm.dylib	0x0000000103d25a16 rustc_codegen_llvm::back::write::write_output_file::h759bbedac00052e7 + 86
16  librustc_codegen_llvm-llvm.dylib	0x0000000103d10325 rustc_codegen_llvm::back::write::codegen::_$u7b$$u7b$closure$u7d$$u7d$::h1c521ce99173cfc9 (.llvm.8720060979851273667) + 1141
17  librustc_codegen_llvm-llvm.dylib	0x0000000103d0f9e3 rustc::util::common::time_ext::h9514d078fc909e9a + 163
18  librustc_codegen_llvm-llvm.dylib	0x0000000103d290fc rustc_codegen_llvm::back::write::codegen::h9f2d7d8d3aa37443 + 3196
19  librustc_codegen_llvm-llvm.dylib	0x0000000103d42a23 rustc_codegen_ssa::back::write::execute_work_item::h7528378acd4d9784 + 499
20  librustc_codegen_llvm-llvm.dylib	0x0000000103d5bb92 std::sys_common::backtrace::__rust_begin_short_backtrace::hfced81a9c03b20d6 + 242
21  librustc_codegen_llvm-llvm.dylib	0x0000000103c524cb std::panicking::try::do_call::h1930a25734e038db (.llvm.14567149245432969043) + 43
22  libstd-148845df6e670d53.dylib 	0x000000010389438f __rust_maybe_catch_panic + 31
23  librustc_codegen_llvm-llvm.dylib	0x0000000103d305c6 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::he0fb440a524af8d4 + 134
24  libstd-148845df6e670d53.dylib 	0x000000010388ad1e _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h2b7e113d1172c209 + 62
25  libstd-148845df6e670d53.dylib 	0x00000001038617de std::sys_common::thread::start_thread::h651378ca55643958 + 142
26  libstd-148845df6e670d53.dylib 	0x0000000103882eb9 std::sys::unix::thread::Thread::new::thread_start::hf3afdbafdcb0fb09 + 9
27  libsystem_pthread.dylib       	0x00007fff72819d76 _pthread_start + 125
28  libsystem_pthread.dylib       	0x00007fff728165d7 thread_start + 15
