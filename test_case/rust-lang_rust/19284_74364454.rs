
rustc: /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/llvm/lib/IR/Instructions.cpp:281: void llvm::CallInst::init(llvm::Value*, llvm::ArrayRef<llvm::Value*>, const llvm::Twine&): Assertion `(i >= FTy->getNumParams() || FTy->getParamType(i) == Args[i]->getType()) && "Calling a function with a bad signature!"' failed.
/home/brian/code/rust/mk/target.mk:165: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core' failed
