
#0  0x00007fabd3b98182 in std::_Function_handler<bool (llvm::GlobalValue const&), llvm::thinLTOInternalizeModule(llvm::Module&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&)::$_2>::_M_invoke(std::_Any_data const&, llvm::GlobalValue const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#0  0x00007fabd3b98182 in std::_Function_handler<bool (llvm::GlobalValue const&), llvm::thinLTOInternalizeModule(llvm::Module&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&)::$_2>::_M_invoke(std::_Any_data const&, llvm::GlobalValue const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00007fabd3bb4b7e in llvm::InternalizePass::maybeInternalize(llvm::GlobalValue&, std::set<llvm::Comdat const*, std::less<llvm::Comdat const*>, std::allocator<llvm::Comdat const*> > const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#2  0x00007fabd3bb4fe6 in llvm::InternalizePass::internalizeModule(llvm::Module&, llvm::CallGraph*) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#3  0x00007fabd3b94199 in llvm::thinLTOInternalizeModule(llvm::Module&, llvm::DenseMap<unsigned long, llvm::GlobalValueSummary*, llvm::DenseMapInfo<unsigned long>, llvm::detail::DenseMapPair<unsigned long, llvm::GlobalValueSummary*> > const&) ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x00007fabd3452558 in LLVMRustPrepareThinLTOInternalize ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x00007fabd3356d2b in rustc_codegen_llvm::back::lto::LtoModuleCodegen::optimize::h509ac79a256a6094 ()
   from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64----Type <return> to continue, or q <return> to quit---
