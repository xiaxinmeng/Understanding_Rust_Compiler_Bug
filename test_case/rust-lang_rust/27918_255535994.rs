
rustc 1.14.0-nightly (6e8f92f11 2016-10-07)
warning: broken MIR ((_11.0: &'static [u8; 0])): bad field access (&[u8]: &'static [u8; 0]): Sorts(ExpectedFound { expected: [u8; 0], found: [u8] })
 --> <anon>:9:10
  |
9 |         (EMPTY, Some(EMPTY))  => (),
  |          ^^^^^

warning: broken MIR ((((_11.1: std::option::Option<&'<empty> [u8]>) as Some).0: &'static [u8; 0])): bad field access (&'<empty> [u8]: &'static [u8; 0]): Sorts(ExpectedFound { expected: [u8; 0], found: [u8] })
 --> <anon>:9:22
  |
9 |         (EMPTY, Some(EMPTY))  => (),
  |                      ^^^^^

rustc: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/llvm/lib/IR/Instructions.cpp:2759: static llvm::CastInst* llvm::CastInst::CreatePointerCast(llvm::Value*, llvm::Type*, const llvm::Twine&, llvm::Instruction*): Assertion `S->getType()->isPtrOrPtrVectorTy() && "Invalid cast"' failed.
Aborted (core dumped)
