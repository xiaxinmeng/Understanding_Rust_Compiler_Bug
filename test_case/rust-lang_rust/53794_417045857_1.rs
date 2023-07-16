
rustc foo.rs --target=x86_64-fuchsia -Clinker=clang/bin/clang -Clink-arg=--target=x86_64-fuchsia -Clink-arg=--sysroot=./sdk/arch/x64/sysroot -Lnative=./sdk/arch/x64/lib
