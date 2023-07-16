
0:004> k
  *** Stack trace for last set context - .thread/.cxr resets it
 # Child-SP          RetAddr           Call Site
00 00000045`b73fd210 00007ffc`91dc7f61 rustc_codegen_llvm_llvm!llvm::report_fatal_error+0xfb
01 00000045`b73fd300 00007ffc`928e3019 rustc_codegen_llvm_llvm!llvm::report_fatal_error+0x21
02 00000045`b73fd350 00007ffc`930a33ec rustc_codegen_llvm_llvm!llvm::MCRegisterInfo::getCodeViewRegNum+0x149
03 00000045`b73fd390 00007ffc`930a46ea rustc_codegen_llvm_llvm!llvm::CodeViewDebug::calculateRanges+0x1dc
04 00000045`b73fd4a0 00007ffc`930a9ade rustc_codegen_llvm_llvm!llvm::CodeViewDebug::collectVariableInfo+0x2da
05 00000045`b73fd5b0 00007ffc`934f0344 rustc_codegen_llvm_llvm!llvm::CodeViewDebug::endFunctionImpl+0xbe
06 00000045`b73fd630 00007ffc`928f6aeb rustc_codegen_llvm_llvm!llvm::DebugHandlerBase::endFunction+0x34
07 00000045`b73fd660 00007ffc`91f8e186 rustc_codegen_llvm_llvm!llvm::AsmPrinter::EmitFunctionBody+0x103b
08 00000045`b73fdd10 00007ffc`9289f075 rustc_codegen_llvm_llvm!llvm::ARMAsmPrinter::runOnMachineFunction+0x3d6
09 00000045`b73fde90 00007ffc`91ec941d rustc_codegen_llvm_llvm!llvm::MachineFunctionPass::runOnFunction+0x1e5
0a 00000045`b73fdf30 00007ffc`91ec959f rustc_codegen_llvm_llvm!llvm::FPPassManager::runOnFunction+0x1bd
0b 00000045`b73fe000 00007ffc`91ec985e rustc_codegen_llvm_llvm!llvm::FPPassManager::runOnModule+0x5f
0c 00000045`b73fe030 00007ffc`91ec8d11 rustc_codegen_llvm_llvm!llvm::FPPassManager::runOnModule+0x31e
0d 00000045`b73fe100 00007ffc`91db9410 rustc_codegen_llvm_llvm!llvm::legacy::PassManagerImpl::run+0x181
0e 00000045`b73fe150 00007ffc`91c41071 rustc_codegen_llvm_llvm!LLVMRustWriteOutputFile+0x140
0f 00000045`b73fe280 00007ffc`91c462cb rustc_codegen_llvm_llvm!rustc_codegen_llvm::back::write::write_output_file+0x71 [d:\workspace\rust\src\librustc_codegen_llvm\back\write.rs @ 106] 
10 (Inline Function) --------`-------- rustc_codegen_llvm_llvm!rustc_codegen_llvm::back::write::codegen::{{closure}}::{{closure}}+0x40 [d:\workspace\rust\src\librustc_codegen_llvm\back\write.rs @ 776] 
11 (Inline Function) --------`-------- rustc_codegen_llvm_llvm!rustc_codegen_llvm::back::write::codegen::with_codegen+0x70 [d:\workspace\rust\src\librustc_codegen_llvm\back\write.rs @ 653] 
12 00000045`b73fe380 00007ffc`91c44397 rustc_codegen_llvm_llvm!rustc_codegen_llvm::back::write::codegen::{{closure}}+0x33b [d:\workspace\rust\src\librustc_codegen_llvm\back\write.rs @ 775] 
13 00000045`b73fe6a0 00007ffc`91c4d051 rustc_codegen_llvm_llvm!rustc_codegen_llvm::back::write::codegen+0x1227 [d:\workspace\rust\src\librustc_codegen_llvm\back\write.rs @ 709] 
14 00000045`b73fe9f0 00007ffc`91ba8ee0 rustc_codegen_llvm_llvm!rustc_codegen_llvm::back::write::execute_work_item+0x2381 [d:\workspace\rust\src\librustc_codegen_llvm\back\write.rs @ 1373] 
15 (Inline Function) --------`-------- rustc_codegen_llvm_llvm!rustc_codegen_llvm::back::write::spawn_work::{{closure}}+0x34d [d:\workspace\rust\src\librustc_codegen_llvm\back\write.rs @ 2018] 
16 00000045`b73ff180 00007ffc`91baa449 rustc_codegen_llvm_llvm!std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()>+0x380 [d:\workspace\rust\src\libstd\sys_common\backtrace.rs @ 136] 
*** WARNING: Unable to verify checksum for std-6a3ca8e43d641166.dll
17 (Inline Function) --------`-------- rustc_codegen_llvm_llvm!std::thread::{{impl}}::spawn::{{closure}}::{{closure}}+0x19 [d:\workspace\rust\src\libstd\thread\mod.rs @ 409] 
18 (Inline Function) --------`-------- rustc_codegen_llvm_llvm!std::panic::{{impl}}::call_once+0x19 [d:\workspace\rust\src\libstd\panic.rs @ 308] 
19 00000045`b73ff5d0 00007ffc`b576d2c2 rustc_codegen_llvm_llvm!std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()>+0x29 [d:\workspace\rust\src\libstd\panicking.rs @ 312] 
1a 00000045`b73ff7c0 00007ffc`91bd99fa std_6a3ca8e43d641166!panic_unwind::__rust_maybe_catch_panic+0x22 [d:\workspace\rust\src\libpanic_unwind\lib.rs @ 105] 
1b (Inline Function) --------`-------- rustc_codegen_llvm_llvm!std::panic::catch_unwind+0x45 [d:\workspace\rust\src\libstd\panic.rs @ 392] 
1c (Inline Function) --------`-------- rustc_codegen_llvm_llvm!std::thread::{{impl}}::spawn::{{closure}}+0x9c [d:\workspace\rust\src\libstd\thread\mod.rs @ 408] 
1d 00000045`b73ff820 00007ffc`b576a9a5 rustc_codegen_llvm_llvm!alloc::boxed::{{impl}}::call_box<(),closure>+0xea [d:\workspace\rust\src\liballoc\boxed.rs @ 640] 
1e (Inline Function) --------`-------- std_6a3ca8e43d641166!alloc::boxed::{{impl}}::call_once+0x7 [d:\workspace\rust\src\liballoc\boxed.rs @ 650] 
1f (Inline Function) --------`-------- std_6a3ca8e43d641166!std::sys_common::thread::start_thread+0x5f [d:\workspace\rust\src\libstd\sys_common\thread.rs @ 24] 
20 00000045`b73ffc30 00007ffc`e797fea4 std_6a3ca8e43d641166!std::sys::windows::thread::{{impl}}::new::thread_start+0x75 [d:\workspace\rust\src\libstd\sys\windows\thread.rs @ 55] 
21 00000045`b73ffc80 00007ffc`e9dd54f1 kernel32!BaseThreadInitThunk+0x14 [base\win32\client\thread.c @ 64] 
22 00000045`b73ffcb0 00000000`00000000 ntdll!RtlUserThreadStart+0x21 [minkernel\ntdll\rtlstrt.c @ 1163] 
