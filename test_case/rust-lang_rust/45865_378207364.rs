
$ clang-4.0 -g -Os -flto -std=gnu99 -c hello.c
$ clang-4.0 -g -Os -flto -std=gnu99 -c pants.c
$ clang-4.0 -g -Os -flto -std=gnu99 -c socks.c
$ clang-4.0 -g -flto -fuse-ld=gold -Wl hello.o pants.o socks.o -o hello

$ file *.o
hello.o: LLVM IR bitcode
pants.o: LLVM IR bitcode
socks.o: LLVM IR bitcode
