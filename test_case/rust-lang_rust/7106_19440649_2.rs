
$ ./x86_64-apple-darwin/stage2/bin/rustc -Z jit --passes instcombine,scalarrepl-ssa ./foo.rs       
NaN
