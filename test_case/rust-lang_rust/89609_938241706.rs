
llvm::Instruction* const* llvm::SmallVectorTemplateCommon<llvm::Instruction*, void>::reserveForParamAndGetAddressImpl<llvm::SmallVectorTemplateBase<llvm::Instruction*, true> >(llvm::SmallVectorTemplateBase<llvm::Instruction*, true>*, llvm::Instruction* const&, unsigned long) at /checkout/src/llvm-project/llvm/include/llvm/ADT/SmallVector.h:217
 (inlined by) llvm::SmallVectorTemplateBase<llvm::Instruction*, true>::reserveForParamAndGetAddress(llvm::Instruction*&, unsigned long) at /checkout/src/llvm-project/llvm/include/llvm/ADT/SmallVector.h:522
 (inlined by) llvm::SmallVectorTemplateBase<llvm::Instruction*, true>::push_back(llvm::Instruction*) at /checkout/src/llvm-project/llvm/include/llvm/ADT/SmallVector.h:547
 (inlined by) PushDefUseChildren at /checkout/src/llvm-project/llvm/lib/Analysis/ScalarEvolution.cpp:4382
 (inlined by) llvm::ScalarEvolution::forgetLoop(llvm::Loop const*) at /checkout/src/llvm-project/llvm/lib/Analysis/ScalarEvolution.cpp:7487
