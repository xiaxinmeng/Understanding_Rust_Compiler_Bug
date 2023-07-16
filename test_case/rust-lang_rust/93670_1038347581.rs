
--------------------------------------------------------------------------------
Ir         
--------------------------------------------------------------------------------
18,075,777  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir          file:function
--------------------------------------------------------------------------------
22,240,315  ???:llvm::AttributeList::addAttribute(llvm::LLVMContext&, unsigned int, llvm::Attribute) const
-4,727,728  ???:llvm::Attribute::hasParentContext(llvm::LLVMContext&) const
 4,724,839  ???:<alloc::vec::drain_filter::DrainFilter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, rustc_trait_selection::traits::project::opt_normalize_projection_type::{closure
-4,400,231  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
 4,293,104  ???:(anonymous namespace)::Verifier::verifyFunctionAttrs(llvm::FunctionType*, llvm::AttributeList, llvm::Value const*, bool)
 3,962,446  ???:llvm::FoldingSet<llvm::AttributeImpl>::NodeEquals(llvm::FoldingSetBase const*, llvm::FoldingSetBase::Node*, llvm::FoldingSetNodeID const&, unsigned int, llvm::FoldingSetNodeID&)
-2,954,144  ???:(anonymous namespace)::Verifier::visitMDNode(llvm::MDNode const&, (anonymous namespace)::Verifier::AreDebugLocsAllowed)
 2,299,517  ???:llvm::Attribute::get(llvm::LLVMContext&, llvm::Attribute::AttrKind, unsigned long)
-2,273,112  ???:llvm::FPPassManager::runOnFunction(llvm::Function&)
 1,577,122  ./string/../sysdeps/x86_64/multiarch/memcmp-avx2-movbe.S:__memcmp_avx2_movbe
-1,401,896  ./nptl/../nptl/pthread_mutex_trylock.c:pthread_mutex_trylock
-1,216,700  ???:llvm::AnalysisResolver::getAnalysisIfAvailable(void const*) const
  -916,235  ???:llvm::MCAssembler::layout(llvm::MCAsmLayout&)
...
