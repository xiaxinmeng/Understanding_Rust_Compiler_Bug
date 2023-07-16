
* thread #2: tid = 0xd98a10, 0x0000000101d9fed4 librustc_llvm-6eb85298.dylib`llvm::X86TargetLowering::isZExtFree(llvm::SDValue, llvm::EVT) const + 4, name = 'rustc', stop reason = EXC_BAD_ACCESS (code=1, address=0x3a)
  * frame #0: 0x0000000101d9fed4 librustc_llvm-6eb85298.dylib`llvm::X86TargetLowering::isZExtFree(llvm::SDValue, llvm::EVT) const + 4
    frame #1: 0x0000000101f900d1 librustc_llvm-6eb85298.dylib`llvm::RegsForValue::getCopyToRegs(llvm::SDValue, llvm::SelectionDAG&, llvm::SDLoc const&, llvm::SDValue&, llvm::SDValue*, llvm::Value const*, llvm::ISD::NodeType) const + 689
    frame #2: 0x0000000101fb0fed librustc_llvm-6eb85298.dylib`llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite) + 8605
    frame #3: 0x0000000101f93d48 librustc_llvm-6eb85298.dylib`llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) + 1144
    frame #4: 0x0000000102025248 librustc_llvm-6eb85298.dylib`llvm::SelectionDAGISel::SelectBasicBlock(llvm::ilist_iterator<llvm::Instruction const>, llvm::ilist_iterator<llvm::Instruction const>, bool&) + 40
    frame #5: 0x00000001020249e2 librustc_llvm-6eb85298.dylib`llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) + 10146
    frame #6: 0x0000000102021174 librustc_llvm-6eb85298.dylib`llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 1684
    frame #7: 0x0000000101d26134 librustc_llvm-6eb85298.dylib`(anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) + 20
    frame #8: 0x000000010221f7e5 librustc_llvm-6eb85298.dylib`llvm::MachineFunctionPass::runOnFunction(llvm::Function&) + 293
    frame #9: 0x0000000102b189d3 librustc_llvm-6eb85298.dylib`llvm::FPPassManager::runOnFunction(llvm::Function&) + 515
    frame #10: 0x0000000102b18c2b librustc_llvm-6eb85298.dylib`llvm::FPPassManager::runOnModule(llvm::Module&) + 43
    frame #11: 0x0000000102b191b3 librustc_llvm-6eb85298.dylib`llvm::legacy::PassManagerImpl::run(llvm::Module&) + 1075
    frame #12: 0x000000010151b240 librustc_llvm-6eb85298.dylib`LLVMRustWriteOutputFile + 368
    frame #13: 0x00000001002164ef librustc_trans-6eb85298.dylib`rustc_trans::back::write::write_output_file::h7e8cf29735c33835 + 95
    frame #14: 0x00000001002be680 librustc_trans-6eb85298.dylib`rustc_trans::back::write::optimize_and_codegen::_$u7b$$u7b$closure$u7d$$u7d$::h368ad232bd936447 + 672
    frame #15: 0x000000010021de92 librustc_trans-6eb85298.dylib`rustc_trans::back::write::execute_work_item::h11850bd68a6e29be + 6434
    frame #16: 0x0000000100218c7b librustc_trans-6eb85298.dylib`rustc_trans::back::write::run_passes::h17498b78d8227324 + 3659
    frame #17: 0x00000001000ba57a librustc_driver-6eb85298.dylib`rustc_driver::driver::phase_5_run_llvm_passes::h11938d38ef5db27c + 298
    frame #18: 0x00000001000a54c3 librustc_driver-6eb85298.dylib`rustc_driver::driver::compile_input::h5b63ccd49eeeb98b + 7699
    frame #19: 0x00000001000cd179 librustc_driver-6eb85298.dylib`rustc_driver::run_compiler::h5441e14d75f46685 + 2041
    frame #20: 0x000000010000dba1 librustc_driver-6eb85298.dylib`std::panicking::try::do_call::h75a39c92a39808cb + 161
    frame #21: 0x00000001041bbfbb libstd-6eb85298.dylib`__rust_maybe_catch_panic + 27
    frame #22: 0x000000010002d585 librustc_driver-6eb85298.dylib`_$LT$F$u20$as$u20$alloc..boxed
