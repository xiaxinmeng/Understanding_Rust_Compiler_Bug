
rustc: /path/to/rust/src/llvm/include/llvm/Support/Casting.h:237: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::IntegerType; Y = llvm::Type; typename llvm::cast_retty<X, Y*>::ret_type = llvm::IntegerType*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
