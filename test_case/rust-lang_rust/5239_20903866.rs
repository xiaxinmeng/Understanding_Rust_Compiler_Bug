
rustc: ../../../../../src/llvm/include/llvm/Support/Casting.h:231: typename enable_if<is_same<Y, typename simplify_type<Y>::SimpleType>, typename cast_retty<X, Y *>::ret_type>::type llvm::cast(Y *) [X = llvm::PointerType, Y = llvm::Type]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
zsh: abort (core dumped)  rustc foo.rs
