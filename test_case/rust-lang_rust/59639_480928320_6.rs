
$ rustc +nightly-2019-04-05 --emit=llvm-ir issue48229.rs -O && grep -C3 trap issue48229.ll
