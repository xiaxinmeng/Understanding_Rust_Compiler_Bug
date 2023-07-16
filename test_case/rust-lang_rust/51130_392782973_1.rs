
#3  0x00007ffff729dc82 in __GI___assert_fail (
    assertion=0x7fffee147b98 "Val && \"isa<> used on a null pointer\"", 
    file=0x7fffee147b58 "/home/nikic/rust/src/llvm/include/llvm/Support/Casting.h", line=92, 
    function=0x7fffee463e00 <llvm::isa_impl_cl<llvm::ConstantSDNode, llvm::SDNode*>::doit(llvm::SDNode const*)::__PRETTY_FUNCTION__> "static bool llvm::isa_impl_cl<To, From*>::doit(const From*) [with To = llvm::ConstantSDNode; From = llvm::SDNode]") at assert.c:101
#4  0x00007fffec3b61a4 in llvm::cast_retty<llvm::ConstantSDNode, llvm::SDValue>::ret_type llvm::dyn_cast<llvm::ConstantSDNode, llvm::SDValue>(llvm::SDValue&) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fffec44762a in llvm::X86TargetLowering::LowerAsmOperandForConstraint(llvm::SDValue, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >&, std::vector<llvm::SDValue, std::allocator<llvm::SDValue> >&, llvm::SelectionDAG&) const ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x00007fffed237f08 in llvm::SelectionDAGBuilder::visitInlineAsm(llvm::ImmutableCallSite) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x00007fffed2451d7 in llvm::SelectionDAGBuilder::visitCall(llvm::CallInst const&) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x00007fffed24c723 in llvm::SelectionDAGBuilder::visit(llvm::Instruction const&) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x00007fffed28fd69 in llvm::SelectionDAGISel::SelectBasicBlock(llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, true, false, void>, false, true>, llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, true, false, void>, false, true>, bool&) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x00007fffed2956d1 in llvm::SelectionDAGISel::SelectAllBasicBlocks(llvm::Function const&) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x00007fffed2973e2 in llvm::SelectionDAGISel::runOnMachineFunction(llvm::MachineFunction&) [clone .part.876] [clone .constprop.902] ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x00007fffec3c07c4 in (anonymous namespace)::X86DAGToDAGISel::runOnMachineFunction(llvm::MachineFunction&) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x00007fffed510775 in llvm::MachineFunctionPass::runOnFunction(llvm::Function&) ()
   from /home/nikic/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x00007fffedecc2c3 in llvm::FPPassManager::runOnFunction(llvm::Function&) ()
