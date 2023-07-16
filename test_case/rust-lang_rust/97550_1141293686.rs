
$ echo 'fn main(){}' | rustc -Cdebuginfo=2 - && strings rust_out | rg "rustc version"
clang LLVM (rustc version 1.62.0-beta.2 (daf68b1f7 2022-05-19))
