
$ clang -target armv5te-unknown-linux-musl -mcpu=arm926ej-s -mfloat-abi=soft --sysroot=$HOME/arm-buildroot-linux-musleabi/sysroot -Os -S -emit-llvm main.c -o main_clang.ll
