
    clang -g banana/test.c -o banana/out -ffile-prefix-map=/tmp/banana= -S -emit-llvm
    