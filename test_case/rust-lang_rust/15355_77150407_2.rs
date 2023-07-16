
(lldb) run ./test.rs
Process 52461 launched: '/usr/local/bin/rustc' (x86_64)
test.rs:4:9: 4:10 warning: unused variable: `n`, #[warn(unused_variables)] on by default
test.rs:4     let n = f as u64 as f64;
                  ^
Process 52461 stopped
* thread #2: tid = 0x126ff8, 0x00000001024efc59 librustc_llvm-4e7c5e5c.dylib`llvm::TypeFinder::run(llvm::Module const&, bool) + 73, name = 'rustc', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x00000001024efc59 librustc_llvm-4e7c5e5c.dylib`llvm::TypeFinder::run(llvm::Module const&, bool) + 73
librustc_llvm-4e7c5e5c.dylib`llvm::TypeFinder::run(llvm::Module const&, bool) + 73:
-> 0x1024efc59:  movq   0x8(%rbx), %rsi
   0x1024efc5d:  testq  %rsi, %rsi
   0x1024efc60:  je     0x1024f0070               ; llvm::TypeFinder::run(llvm::Module const&, bool) + 1120
   0x1024efc66:  movzbl 0x8(%rsi), %eax
(lldb) bt
* thread #2: tid = 0x126ff8, 0x00000001024efc59 librustc_llvm-4e7c5e5c.dylib`llvm::TypeFinder::run(llvm::Module const&, bool) + 73, name = 'rustc', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
  * frame #0: 0x00000001024efc59 librustc_llvm-4e7c5e5c.dylib`llvm::TypeFinder::run(llvm::Module const&, bool) + 73
    frame #1: 0x00000001023b4192 librustc_llvm-4e7c5e5c.dylib`llvm::TypePrinting::incorporateTypes(llvm::Module const&) + 34
    frame #2: 0x00000001023c430c librustc_llvm-4e7c5e5c.dylib`llvm::Value::printAsOperand(llvm::raw_ostream&, bool, llvm::Module const*) const + 364
    frame #3: 0x0000000101d42bfa librustc_llvm-4e7c5e5c.dylib`llvm::AsmPrinter::lowerConstant(llvm::Constant const*) + 1610
    frame #4: 0x0000000101d45842 librustc_llvm-4e7c5e5c.dylib`emitGlobalConstantImpl(llvm::Constant const*, llvm::AsmPrinter&) + 2498
    frame #5: 0x0000000101d3dfcb librustc_llvm-4e7c5e5c.dylib`llvm::AsmPrinter::EmitGlobalVariable(llvm::GlobalVariable const*) + 1899
    frame #6: 0x0000000101d418f0 librustc_llvm-4e7c5e5c.dylib`llvm::AsmPrinter::doFinalization(llvm::Module&) + 80
    frame #7: 0x00000001024cd523 librustc_llvm-4e7c5e5c.dylib`llvm::FPPassManager::doFinalization(llvm::Module&) + 83
    frame #8: 0x00000001024cdafd librustc_llvm-4e7c5e5c.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 1421
    frame #9: 0x00000001013b68c7 librustc_llvm-4e7c5e5c.dylib`LLVMRustWriteOutputFile(Target=<unavailable>, PMR=0x0000000103f05e70, M=0x0000000103f05e70, path=<unavailable>, FileType=CGFT_ObjectFile) + 503 at PassWrapper.cpp:211
    frame #10: 0x00000001005a34ff librustc_trans-4e7c5e5c.dylib`back::write::write_output_file::hdb56d5084c63bc86v6b + 223
    frame #11: 0x00000001005a601f librustc_trans-4e7c5e5c.dylib`back::write::optimize_and_codegen::closure.38410 + 1327
    frame #12: 0x00000001005b0cb9 librustc_trans-4e7c5e5c.dylib`back::write::execute_work_item::hd8db7afca5e1ebc8nZc + 6345
    frame #13: 0x00000001005a7af4 librustc_trans-4e7c5e5c.dylib`back::write::run_passes::hd2f044ab61edb7b9cFc + 6468
    frame #14: 0x0000000100029e1a librustc_driver-4e7c5e5c.dylib`driver::phase_5_run_llvm_passes::hb3416b462f92d001dOa + 362
    frame #15: 0x00000001000055db librustc_driver-4e7c5e5c.dylib`driver::compile_input::heef3a8983ecc9746Iba + 5339
    frame #16: 0x00000001000d644f librustc_driver-4e7c5e5c.dylib`run_compiler::h09dbb9820b7ec68bF5b + 4159
    frame #17: 0x00000001000d3808 librustc_driver-4e7c5e5c.dylib`thunk::F.Invoke$LT$A$C$$u20$R$GT$::invoke::h10490261994158956019 + 600
    frame #18: 0x00000001000d2350 librustc_driver-4e7c5e5c.dylib`rt::unwind::try::try_fn::h10082179094849837244 + 192
    frame #19: 0x0000000103a67089 libstd-4e7c5e5c.dylib`rust_try_inner + 9
    frame #20: 0x0000000103a67076 libstd-4e7c5e5c.dylib`rust_try + 6
    frame #21: 0x00000001000d2b89 librustc_driver-4e7c5e5c.dylib`thunk::F.Invoke$LT$A$C$$u20$R$GT$::invoke::h16026492124957986721 + 1433
    frame #22: 0x00000001039d06f3 libstd-4e7c5e5c.dylib`sys::thread::thread_start::hb52a23d4ffceb69312E + 179
    frame #23: 0x00007fff865272fc libsystem_pthread.dylib`_pthread_body + 131
    frame #24: 0x00007fff86527279 libsystem_pthread.dylib`_pthread_start + 176
