
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:51
#1  0x00007ffff4d0e801 in __GI_abort () at abort.c:79
#2  0x00007ffff4cfe39a in __assert_fail_base (
    fmt=0x7ffff4e857d8 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", 
    assertion=assertion@entry=0x7fffeb283450 "getType()->isPointerTy() && \"Only pointers have byval types\"", 
    file=file@entry=0x7fffeb283210 "/home/nikic/rust/src/llvm-project/llvm/lib/IR/Function.cpp", 
    line=line@entry=117, 
    function=function@entry=0x7fffeb286d40 <llvm::Argument::getParamByValType() const::__PRETTY_FUNCTION__> "llvm::Type* llvm::Argument::getParamByValType() const") at assert.c:92
#3  0x00007ffff4cfe412 in __GI___assert_fail (
    assertion=assertion@entry=0x7fffeb283450 "getType()->isPointerTy() && \"Only pointers have byval types\"", 
    file=file@entry=0x7fffeb283210 "/home/nikic/rust/src/llvm-project/llvm/lib/IR/Function.cpp", 
    line=line@entry=117, 
    function=function@entry=0x7fffeb286d40 <llvm::Argument::getParamByValType() const::__PRETTY_FUNCTION__> "llvm::Type* llvm::Argument::getParamByValType() const") at assert.c:101
#4  0x00007fffe95d8c21 in llvm::Argument::getParamByValType (this=this@entry=0x7fffec4a2c80)
    at /home/nikic/rust/src/llvm-project/llvm/lib/IR/Function.cpp:117
#5  0x00007fffe87f86cd in llvm::SelectionDAGISel::LowerArguments (this=this@entry=0x7fffdc005220, 
    F=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGBuilder.cpp:9554
#6  0x00007fffe8852f25 in llvm::SelectionDAGISel::SelectAllBasicBlocks (
    this=this@entry=0x7fffdc005220, Fn=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:1377
#7  0x00007fffe88547dd in llvm::SelectionDAGISel::runOnMachineFunction (
    this=this@entry=0x7fffdc005220, mf=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:500
#8  0x00007fffe885643c in llvm::SelectionDAGISel::runOnMachineFunction (
    this=this@entry=0x7fffdc005220, mf=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAGISel.cpp:698
#9  0x00007fffe7740edf in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction (
    this=0x7fffdc005220, MF=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/Target/X86/X86ISelDAGToDAG.cpp:194
#10 0x00007fffe8afc0ae in llvm::MachineFunctionPass::runOnFunction (this=0x7fffdc005220, F=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/CodeGen/MachineFunctionPass.cpp:73
#11 0x00007fffe9626b68 in llvm::FPPassManager::runOnFunction (this=this@entry=0x7fffdc0010d0, 
    F=...) at /home/nikic/rust/src/llvm-project/llvm/lib/IR/LegacyPassManager.cpp:1648
#12 0x00007fffe9626c61 in llvm::FPPassManager::runOnModule (this=0x7fffdc0010d0, M=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/IR/LegacyPassManager.cpp:1685
#13 0x00007fffe9625e89 in (anonymous namespace)::MPPassManager::runOnModule (M=..., 
    this=<optimized out>)
    at /home/nikic/rust/src/llvm-project/llvm/lib/IR/LegacyPassManager.cpp:1750
#14 llvm::legacy::PassManagerImpl::run (this=0x7fffdc006380, M=...)
    at /home/nikic/rust/src/llvm-project/llvm/lib/IR/LegacyPassManager.cpp:1863
#15 0x00007fffe765fea2 in LLVMRustWriteOutputFile ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x00007fffe7637c4c in rustc_codegen_llvm::back::write::write_output_file ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
