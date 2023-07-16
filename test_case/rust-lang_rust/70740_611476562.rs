
❯ gcc -fPIE -static-pie hello.c && ./a.out 
main = 0x7f488da7ae09d

❯ gcc -fuse-ld=gold -fPIE -static-pie hello.c && ./a.out 
/usr/bin/ld.gold: --no-dynamic-linker: unknown option
/usr/bin/ld.gold: use the --help option for usage information
collect2: error: ld returned 1 exit status

❯ gcc -fuse-ld=bfd -fPIE -static-pie hello.c && ./a.out 
main = 0x7ff5ffb6be09d

❯ gcc -fuse-ld=lld -fPIE -static-pie hello.c && ./a.out 
Segmentation fault (core dumped)

❯ musl-gcc  -fuse-ld=lld -fPIE -static-pie hello.c && ./a.out 
main = 0x7f88a6fefbe9d

❯ musl-gcc  -fuse-ld=gold -fPIE -static-pie hello.c && ./a.out 
/usr/bin/ld.gold: --no-dynamic-linker: unknown option
/usr/bin/ld.gold: use the --help option for usage information
collect2: error: ld returned 1 exit status

❯ musl-gcc  -fuse-ld=bfd -fPIE -static-pie hello.c && ./a.out 
main = 0x7fb12fb922b9d

❯ musl-gcc -fPIE -static-pie hello.c && ./a.out 
main = 0x7f2c5b37f2b9d

❯ ld.gold -version
GNU gold (version 2.34-2.fc32) 1.16

❯ ld.bfd -version
GNU ld version 2.34-2.fc32
Copyright (C) 2020 Free Software Foundation, Inc.
This program is free software; you may redistribute it under the terms of
the GNU General Public License version 3 or (at your option) a later version.
This program has absolutely no warranty.

❯ ld.lld -version
LLD 10.0.0 (compatible with GNU linkers)
