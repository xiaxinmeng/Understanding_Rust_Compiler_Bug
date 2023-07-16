
llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>::const_pointer llvm::ilist_detail::NodeAccess::getValuePtr<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void> >(llvm::ilist_node_impl<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void> > const*) at /checkout/src/llvm-project/llvm/include/llvm/ADT/ilist_node.h:184
 (inlined by) llvm::ilist_detail::SpecificNodeAccess<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void> >::getValuePtr(llvm::ilist_node_impl<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void> > const*) at /checkout/src/llvm-project/llvm/include/llvm/ADT/ilist_node.h:229
 (inlined by) llvm::ilist_iterator<llvm::ilist_detail::node_options<llvm::Instruction, false, false, void>, true, true>::operator*() const at /checkout/src/llvm-project/llvm/include/llvm/ADT/ilist_iterator.h:139
 (inlined by) llvm::simple_ilist<llvm::Instruction>::back() const at /checkout/src/llvm-project/llvm/include/llvm/ADT/simple_ilist.h:141
 (inlined by) llvm::BasicBlock::getTerminator() const at /checkout/src/llvm-project/llvm/lib/IR/BasicBlock.cpp:151
