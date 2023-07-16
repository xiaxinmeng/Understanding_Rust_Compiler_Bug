
$ RUST_BACKTRACE=1 rustc pr19593.rs
pr19593.rs:1:1: 1:21 warning: struct is never used: `Struct`, #[warn(dead_code)] on by default
pr19593.rs:1 struct Struct([u8]);
             ^~~~~~~~~~~~~~~~~~~~
pr19593.rs:4:1: 7:2 warning: function is never used: `as_slice`, #[warn(dead_code)] on by default
pr19593.rs:4 fn as_slice(s: &Struct) -> &[u8] {
pr19593.rs:5     let &Struct(ref this) = s;
pr19593.rs:6     this
pr19593.rs:7 } 
rustc: /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/include/llvm/Support/Casting.h:237: typename llvm::cast_retty<X, Y*>::ret_type llvm::cast(Y*) [with X = llvm::PointerType; Y = const llvm::Type; typename llvm::cast_retty<X, Y*>::ret_type = const llvm::PointerType*]: Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
