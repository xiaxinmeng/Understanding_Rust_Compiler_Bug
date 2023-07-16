
rustc: /home/steve/src/rust/src/llvm/include/llvm/Support/Casting.h:237: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::GlobalVariable; Y = llvm::Value; typename llvm::cast_retty<X, Y*>::ret_type = llvm::GlobalVariable*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
