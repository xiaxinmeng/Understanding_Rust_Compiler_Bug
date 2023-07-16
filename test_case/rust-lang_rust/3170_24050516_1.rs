
$ rustc foo.rs
foo.rs:2:16: 2:19 error: const index-expr is out of bounds
foo.rs:2 static b: int = a[1];
                         ^~~
Assertion failed: (ReqTy && "extractvalue indices invalid!"), function getExtractValue, file ../../../../src/llvm/lib/IR/Constants.cpp, line 1982.
zsh: abort      rustc foo.rs
