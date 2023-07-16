
$ clang -flto -O3 -c f.c
$ clang -flto -O3 -c world.c
$ clang -fuse-ld=lld -flto -Wl,--plugin-opt=-lto-embed-bitcode=optimized world.o f.o -O3 -g -o world
$ objcopy world --dump-section .llvmbc=bc.bc
$ llvm-dis bc.bc
$ 
