
rustc: /checkout/src/llvm-project/llvm/include/llvm/Support/Casting.h:269: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::Function; Y = llvm::Value; typename llvm::cast_retty<X, Y*>::ret_type = llvm::Function*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
[RUSTC-TIMING] panic_abort test:false 0.094
rustc exited with signal: 6
error: could not compile `panic_abort`
