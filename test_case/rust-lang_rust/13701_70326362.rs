
hello.rs:4:25: 4:31 error: const index-expr is out of bounds
hello.rs:4         static v: int = VEC[6];
                                   ^~~~~~
rustc: /home/steve/src/rust/src/llvm/lib/IR/Constants.cpp:2176: static llvm::Constant* llvm::ConstantExpr::getExtractValue(llvm::Constant*, llvm::ArrayRef<unsigned int>, llvm::Type*): Assertion `ReqTy && "extractvalue indices invalid!"' failed.
