
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/include/llvm/Support/Casting.h:237: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::Function; Y = llvm::Value; typename llvm::cast_retty<X, Y*>::ret_type = llvm::Function*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
Aborted (core dumped)
playpen: application terminated with error code 134
