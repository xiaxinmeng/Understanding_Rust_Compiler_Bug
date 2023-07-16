
$ rustc -L . linenoise.rc
Assertion failed: (isa<X>(Val) && "cast<Ty>() argument of incompatible type!"), function cast, file /Users/at0m13/Documents/rust/src/llvm/include/llvm/Support/Casting.h, line 197.
Abort trap: 6

$ rustc -L . --lib linenoise.rs
warning: missing crate link meta `name`, using `linenoise` as default
warning: missing crate link meta `vers`, using `0.0` as default
warning: no debug symbols in executable (-arch x86_64)
