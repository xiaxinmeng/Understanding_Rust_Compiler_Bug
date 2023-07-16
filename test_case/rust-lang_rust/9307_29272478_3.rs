
rustc: /build/rust-git/src/rust/src/llvm/lib/IR/Instructions.cpp:276: void llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&): Assertion `(Args.size() == FTy->getNumParams() || (FTy->isVarArg() && Args.size() > FTy->getNumParams())) && "Calling a function with bad signature!"' failed.
