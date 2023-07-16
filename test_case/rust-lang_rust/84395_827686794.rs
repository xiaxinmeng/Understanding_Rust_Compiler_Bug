
$ clang -flto -c world.c
$ file world.o
world.o: LLVM IR bitcode
$ r2 world.o
 -- Remember that word: C H A I R
[0x00000000]> /x 4243c0de
Searching 4 bytes in [0x0-0xc08]
hits: 1
0x00000000 hit0_0 4243c0de
