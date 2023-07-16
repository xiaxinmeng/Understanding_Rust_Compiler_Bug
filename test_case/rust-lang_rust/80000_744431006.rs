
$ musl-gcc -pie hello.c -O2
$ file a.out
a.out: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-x86_64.so.1, with debug_info, not stripped
$ readelf -S a.out | grep dynamic
  [17] .dynamic          DYNAMIC          0000000000003df0  00002df0
