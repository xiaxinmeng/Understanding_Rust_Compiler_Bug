
$ gdb782 --args x86_64-unknown-freebsd/stage0/bin/rustc --cfg stage0  -O --cfg rtopt -C rpath -C prefer-dynamic -C no-stack-check --target=x86_64-unknown-freebsd   -L "x86_64-unknown-freebsd/rt" -L native="/exps/rust/x86_64-unknown-freebsd/llvm/Release/lib"     --out-dir x86_64-unknown-freebsd/stage0/lib/rustlib/x86_64-unknown-freebsd/lib -C extra-filename=-9026086f src/libcore/lib.rs
