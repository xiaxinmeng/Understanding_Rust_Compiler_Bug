
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/IR/Instructions.cpp:550: void llvm::InvokeInst::init(llvm::Value*, llvm::BasicBlock*, llvm::BasicBlock*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&): Assertion `(i >= FTy->getNumParams() || FTy->getParamType(i) == Args[i]->getType()) && "Invoking a function with a bad signature!"' failed.
[1]    8065 abort (core dumped)  rustc 20605.rs
