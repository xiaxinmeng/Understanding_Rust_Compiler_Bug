
#0  0x00007ffff7383428 in __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:54
#1  0x00007ffff738502a in __GI_abort () at abort.c:89
#2  0x00007ffff737bbd7 in __assert_fail_base (fmt=<optimised out>, assertion=assertion@entry=0x7ffff3224b18 "isImm() && \"Wrong MachineOperand accessor\"", 
    file=file@entry=0x7ffff3224848 "/home/aidanhs/Desktop/rust/rust-large/src/llvm/include/llvm/CodeGen/MachineOperand.h", line=line@entry=411, 
    function=function@entry=0x7ffff32e4d60 <_ZZNK4llvm14MachineOperand6getImmEvE19__PRETTY_FUNCTION__> "int64_t llvm::MachineOperand::getImm() const") at assert.c:92
#3  0x00007ffff737bc82 in __GI___assert_fail (assertion=assertion@entry=0x7ffff3224b18 "isImm() && \"Wrong MachineOperand accessor\"", 
    file=file@entry=0x7ffff3224848 "/home/aidanhs/Desktop/rust/rust-large/src/llvm/include/llvm/CodeGen/MachineOperand.h", line=line@entry=411, 
    function=function@entry=0x7ffff32e4d60 <_ZZNK4llvm14MachineOperand6getImmEvE19__PRETTY_FUNCTION__> "int64_t llvm::MachineOperand::getImm() const") at assert.c:101
#4  0x00007ffff2195186 in llvm::MachineOperand::getImm (this=<optimised out>)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/include/llvm/CodeGen/MachineOperand.h:411
#5  llvm::X86InstrInfo::convertToThreeAddress (this=0x7fffec222c20, MFI=..., MI=..., LV=0x7fffed541a80)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/Target/X86/X86InstrInfo.cpp:3082
#6  0x00007ffff2881cb7 in (anonymous namespace)::TwoAddressInstructionPass::convertInstTo3Addr (Dist=5, RegB=2147483648, RegA=2147483649, nmi=..., mi=..., 
    this=0x7fffed541c00) at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/CodeGen/TwoAddressInstructionPass.cpp:707
#7  (anonymous namespace)::TwoAddressInstructionPass::tryInstructionTransform (this=this@entry=0x7fffed541c00, mi=..., nmi=..., SrcIdx=1, DstIdx=<optimised out>, 
    Dist=5, shouldOnlyCommute=false) at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/CodeGen/TwoAddressInstructionPass.cpp:1273
#8  0x00007ffff2885143 in (anonymous namespace)::TwoAddressInstructionPass::runOnMachineFunction (this=<optimised out>, Func=...)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/CodeGen/TwoAddressInstructionPass.cpp:1686
#9  0x00007ffff2703aa5 in llvm::MachineFunctionPass::runOnFunction (this=0x7fffed541c00, F=...)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/CodeGen/MachineFunctionPass.cpp:60
#10 0x00007ffff2f56ecb in llvm::FPPassManager::runOnFunction (this=0x7fffed4ef880, F=...)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/IR/LegacyPassManager.cpp:1523
#11 0x00007ffff2f572eb in llvm::FPPassManager::runOnModule (this=0x7fffed4ef880, M=...)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/IR/LegacyPassManager.cpp:1544
#12 0x00007ffff2f56acd in (anonymous namespace)::MPPassManager::runOnModule (M=..., this=<optimised out>)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/IR/LegacyPassManager.cpp:1600
#13 llvm::legacy::PassManagerImpl::run (this=0x7fffed4bb300, M=...) at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/IR/LegacyPassManager.cpp:1703
#14 0x00007ffff2f56cc9 in llvm::legacy::PassManager::run (this=this@entry=0x7fffed5c8400, M=...)
    at /home/aidanhs/Desktop/rust/rust-large/src/llvm/lib/IR/LegacyPassManager.cpp:1734
#15 0x00007ffff203bf4f in LLVMRustWriteOutputFile (Target=0x7fffec217ec0, PMR=0x7fffed5c8400, M=0x7fffed459600, Path=<optimised out>, RustFileType=<optimised out>)
    at ../rustllvm/PassWrapper.cpp:470
#16 0x00007ffff6dae2e4 in rustc_trans::back::write::write_output_file::h4eabd5e87bd8b06b ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_trans-64bf574ec80a57c3.so
#17 0x00007ffff6db2fdf in rustc_trans::back::write::optimize_and_codegen::_$u7b$$u7b$closure$u7d$$u7d$::h90577f4377253093 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_trans-64bf574ec80a57c3.so
#18 0x00007ffff6db1b70 in rustc_trans::back::write::optimize_and_codegen::h6d075ca45fad08bb ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_trans-64bf574ec80a57c3.so
#19 0x00007ffff6db57dd in rustc_trans::back::write::execute_work_item::hcc36a5ceda537abc ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_trans-64bf574ec80a57c3.so
#20 0x00007ffff6db60ae in rustc_trans::back::write::run_work_singlethreaded::h5f9b58b191ca2169 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_trans-64bf574ec80a57c3.so
#21 0x00007ffff6db3c71 in rustc_trans::back::write::run_passes::hfdcff321ca143e00 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_trans-64bf574ec80a57c3.so
#22 0x00007ffff7aec06b in rustc_driver::driver::phase_5_run_llvm_passes::h32e533cbc3a7bad9 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-571fa3485abbcd87.so
#23 0x00007ffff7ad3b0b in rustc_driver::driver::compile_input::h11cc0ffcaa61b863 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-571fa3485abbcd87.so
#24 0x00007ffff7a7a7a2 in rustc_driver::run_compiler::ha164066ecfb116c6 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-571fa3485abbcd87.so
---Type <return> to continue, or q <return> to quit---
#25 0x00007ffff7a6ea0c in std::panicking::try::do_call::h48abe9f03a823d2d ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-571fa3485abbcd87.so
#26 0x00007ffff77a8c37 in __rust_maybe_catch_panic ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-f04b25e6ba18446f.so
#27 0x00007ffff7a76613 in _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h8329aa22c9802433 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-571fa3485abbcd87.so
#28 0x00007ffff779e891 in std::sys::imp::thread::Thread::new::thread_start::hfa23ef22483cb516 ()
   from /home/aidanhs/Desktop/rust/rust-large/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/libstd-f04b25e6ba18446f.so
#29 0x00007ffff02e56ba in start_thread (arg=0x7fffee7ff700) at pthread_create.c:333
#30 0x00007ffff745482d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
