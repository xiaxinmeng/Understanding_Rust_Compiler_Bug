
<anon>:3:18: 3:22 error: const index-expr is out of bounds
<anon>:3 const B: usize = A[1];
                          ^~~~
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/IR/Constants.cpp:2174: static llvm::Constant* llvm::ConstantExpr::getExtractValue(llvm::Constant*, llvm::ArrayRef<unsigned int>, llvm::Type*): Assertion `ReqTy && "extractvalue indices invalid!"' failed.
Aborted (core dumped)
playpen: application terminated with error code 134
