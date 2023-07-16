 bash
#!/bin/bash

# new rustup.rs nightly darwin toolchain
DIR=/home/m4b/.multirust/toolchains/nightly/lib/rustlib/x86_64-apple-darwin/lib
# assumes you've got the darwin system binaries from `/usr/lib/` hanging out here (change this to wherever they might be)
DARWIN_PATH=/home/m4b/binaries/darwin/usr/lib
SYS=$DARWIN_PATH/system
# i built lld from source, and this is the path to it
LLD=/home/m4b/Downloads/llvm-3.8.0.src/build/bin/lld

MAIN=$1
NAME=${MAIN%.*}

rustc --target=x86_64-apple-darwin $MAIN

$LLD -flavor darwin -arch x86_64 -t "$SYS/libsystem_m.dylib" "$SYS/libsystem_kernel.dylib" "$SYS/libsystem_info.dylib" "$SYS/libsystem_malloc.dylib" "$SYS/libsystem_platform.dylib" "$SYS/libsystem_c.dylib" "$SYS/libsystem_pthread.dylib" "$SYS/libunwind.dylib" "$SYS/libdyld.dylib" "$DIR/liblibc-18402db3.rlib" "$DIR/libstd-18402db3.rlib" "$DIR/libcollections-18402db3.rlib" "$DIR/librustc_unicode-18402db3.rlib" "$DIR/librand-18402db3.rlib" "$DIR/liballoc-18402db3.rlib" "$DIR/liballoc_jemalloc-18402db3.rlib" "$DIR/libcore-18402db3.rlib" "$DIR/libcompiler-rt.a"  ${NAME}.0.o -o ${NAME}_darwin
