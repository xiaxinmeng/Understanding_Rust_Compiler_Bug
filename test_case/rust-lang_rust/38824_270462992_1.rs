
$ rustc --target nvptx64-nvidia-cuda foo.rs
rustc: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/Target/NVPTX/NVPTXISelLowering.cpp:2492: virtualllvm::SDValue llvm::NVPTXTargetLowering::LowerReturn(llvm::SDValue, llvm::CallingConv::ID, bool, const llvm::SmallVectorImpl<llvm::ISD::OutputArg>&, const llvm::SmallVectorImpl<llvm::SDValue>&, const llvm::SDLoc&, llvm::SelectionDAG&) const: Assertion `ValVTs.size() == OutVals.size() && "Bad return value decomposition"' failed.
