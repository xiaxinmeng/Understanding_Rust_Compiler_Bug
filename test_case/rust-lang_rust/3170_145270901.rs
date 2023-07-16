
<anon>:2:17: 2:21 error: const index-expr is out of bounds
<anon>:2 static b: i32 = a[1];
                         ^~~~
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/include/llvm/Support/Casting.h:237: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::SequentialType; Y = llvm::Type; typename llvm::cast_retty<X, Y*>::ret_type = llvm::SequentialType*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
Aborted (core dumped)
playpen: application terminated with error code 134
