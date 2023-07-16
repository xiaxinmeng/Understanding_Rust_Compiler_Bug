
rustc: .../llvm-project/llvm/include/llvm/Support/Casting.h:264: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::Function; Y = llvm::Value; typename llvm::cast_retty<X, Y*>::ret_type = llvm::Function*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
