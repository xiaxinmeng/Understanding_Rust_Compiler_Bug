
$ rustc +nightly-2019-04-05 --emit=llvm-ir issue48227.rs -O && grep -C3 trap issue48227.ll
