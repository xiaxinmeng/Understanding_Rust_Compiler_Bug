
rustc: /home/keegan/rust-master/src/llvm/include/llvm/Support/Casting.h:237: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::PointerType; Y = const llvm::Type; typename llvm::cast_retty<X, Y*>::ret_type = const llvm::PointerType*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
