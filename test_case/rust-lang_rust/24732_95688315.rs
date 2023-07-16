
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/IR/DataLayout.cpp:655: unsigned int llvm::DataLayout::getAlignment(llvm::Type*, bool) const: Assertion `Ty->isSized() && "Cannot getTypeInfo() on a type that is unsized!"' failed.
fish: Job 2, “rustc test.rs ” terminated by signal SIGABRT (Abort)
