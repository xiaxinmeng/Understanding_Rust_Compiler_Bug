rust
$ cargo +alt-2017-12-04 build --release -p style --manifest-path ports/servo/Cargo.toml -v
[…]
rustc: /checkout/src/llvm/include/llvm/Support/Casting.h:236: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::IntegerType; Y = llvm::Type; typename llvm::cast_retty<X, Y*>::ret_type = llvm::IntegerType*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
error: Could not compile `style`.

Caused by:
  process didn't exit successfully: `rustc --crate-name style […]` (signal: 6, SIGABRT: process abort signal)
