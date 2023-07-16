
$ clang-4.0 -flto -c hello.c
$ file hello.o
hello.o: LLVM IR bitcode

$ clang-4.0 -flto -fuse-ld=gold -Wl hello.o pants.o socks.o
$ file a.out
a.out: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 2.6.32, not stripped
