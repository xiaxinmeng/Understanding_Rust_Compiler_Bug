
root@fa08d8342649:/tmp# cat test.c 
#include<stdio.h>
int main() { 
   printf("hello, world");
   return 0;
}
root@fa08d8342649:/tmp# /musl-armv5te/bin/musl-gcc test.c  -v
Using built-in specs.
Reading specs from /musl-armv5te/lib/musl-gcc.specs
rename spec cpp_options to old_cpp_options
COLLECT_GCC=arm-linux-gnueabi-gcc
COLLECT_LTO_WRAPPER=/usr/lib/gcc-cross/arm-linux-gnueabi/9/lto-wrapper
Target: arm-linux-gnueabi
Configured with: ../src/configure -v --with-pkgversion='Ubuntu 9.3.0-17ubuntu1~20.04' --with-bugurl=file:///usr/share/doc/gcc-9/README.Bugs --enable-languages=c,ada,c++,go,d,fortran,objc,obj-c++,gm2 --prefix=/usr --with-gcc-major-version-only --program-suffix=-9 --enable-shared --enable-linker-build-id --libexecdir=/usr/lib --without-included-gettext --enable-threads=posix --libdir=/usr/lib --enable-nls --with-sysroot=/ --enable-clocale=gnu --enable-libstdcxx-debug --enable-libstdcxx-time=yes --with-default-libstdcxx-abi=new --enable-gnu-unique-object --disable-libitm --disable-libquadmath --disable-libquadmath-support --enable-plugin --with-system-zlib --without-target-system-zlib --enable-libpth-m2 --enable-multiarch --enable-multilib --disable-sjlj-exceptions --with-specs='%{mfloat-abi=hard:-march=armv7-a -mcpu=generic-armv7-a -mfloat-abi=hard}' --with-arch=armv5t --with-float=soft --disable-werror --enable-multilib --enable-checking=release --build=x86_64-linux-gnu --host=x86_64-linux-gnu --target=arm-linux-gnueabi --program-prefix=arm-linux-gnueabi- --includedir=/usr/arm-linux-gnueabi/include
Thread model: posix
gcc version 9.3.0 (Ubuntu 9.3.0-17ubuntu1~20.04) 
COLLECT_GCC_OPTIONS='-v' '-specs=/musl-armv5te/lib/musl-gcc.specs'  '-mfloat-abi=soft' '-mtls-dialect=gnu' '-marm' '-march=armv5t'
 /usr/lib/gcc-cross/arm-linux-gnueabi/9/cc1 -quiet -v -imultiarch arm-linux-gnueabi test.c -nostdinc -isystem /musl-armv5te/include -isystem /usr/lib/gcc-cross/arm-linux-gnueabi/9/include -quiet -dumpbase test.c -mfloat-abi=soft -mtls-dialect=gnu -marm -march=armv5t -auxbase test -version -fstack-protector-strong -Wformat -Wformat-security -o /tmp/ccjzQdVu.s
GNU C17 (Ubuntu 9.3.0-17ubuntu1~20.04) version 9.3.0 (arm-linux-gnueabi)
	compiled by GNU C version 9.3.0, GMP version 6.2.0, MPFR version 4.0.2, MPC version 1.1.0, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
#include "..." search starts here:
#include <...> search starts here:
 /musl-armv5te/include
 /usr/lib/gcc-cross/arm-linux-gnueabi/9/include
End of search list.
GNU C17 (Ubuntu 9.3.0-17ubuntu1~20.04) version 9.3.0 (arm-linux-gnueabi)
	compiled by GNU C version 9.3.0, GMP version 6.2.0, MPFR version 4.0.2, MPC version 1.1.0, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
Compiler executable checksum: 08edcb181c35d21fb3d1fe33a97cd5f3
COLLECT_GCC_OPTIONS='-v' '-specs=/musl-armv5te/lib/musl-gcc.specs'  '-mfloat-abi=soft' '-mtls-dialect=gnu' '-marm' '-march=armv5t'
 /usr/lib/gcc-cross/arm-linux-gnueabi/9/../../../../arm-linux-gnueabi/bin/as -v -march=armv5t -mfloat-abi=soft -meabi=5 -o /tmp/cc2eOd6u.o /tmp/ccjzQdVu.s
GNU assembler version 2.34 (arm-linux-gnueabi) using BFD version (GNU Binutils for Ubuntu) 2.34
COMPILER_PATH=/usr/lib/gcc-cross/arm-linux-gnueabi/9/:/usr/lib/gcc-cross/arm-linux-gnueabi/9/:/usr/lib/gcc-cross/arm-linux-gnueabi/:/usr/lib/gcc-cross/arm-linux-gnueabi/9/:/usr/lib/gcc-cross/arm-linux-gnueabi/:/usr/lib/gcc-cross/arm-linux-gnueabi/9/../../../../arm-linux-gnueabi/bin/
LIBRARY_PATH=/usr/lib/gcc-cross/arm-linux-gnueabi/9/:/usr/lib/gcc-cross/arm-linux-gnueabi/9/../../../../arm-linux-gnueabi/lib/../lib/:/lib/arm-linux-gnueabi/:/lib/../lib/:/usr/lib/arm-linux-gnueabi/:/usr/lib/../lib/:/usr/lib/gcc-cross/arm-linux-gnueabi/9/../../../../arm-linux-gnueabi/lib/:/lib/:/usr/lib/
COLLECT_GCC_OPTIONS='-v' '-specs=/musl-armv5te/lib/musl-gcc.specs'  '-mfloat-abi=soft' '-mtls-dialect=gnu' '-marm' '-march=armv5t'
 /usr/lib/gcc-cross/arm-linux-gnueabi/9/collect2 -plugin /usr/lib/gcc-cross/arm-linux-gnueabi/9/liblto_plugin.so -plugin-opt=/usr/lib/gcc-cross/arm-linux-gnueabi/9/lto-wrapper -plugin-opt=-fresolution=/tmp/ccpsLUOq.res -plugin-opt=-pass-through=/usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc.a -plugin-opt=-pass-through=/usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc_eh.a -plugin-opt=-pass-through=-lc -plugin-opt=-pass-through=/usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc.a -plugin-opt=-pass-through=/usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc_eh.a -dynamic-linker /lib/ld-musl-arm.so.1 -nostdlib -z relro /musl-armv5te/lib/Scrt1.o /musl-armv5te/lib/crti.o /usr/lib/gcc-cross/arm-linux-gnueabi/9/crtbeginS.o -L/musl-armv5te/lib -L /usr/lib/gcc-cross/arm-linux-gnueabi/9/. /tmp/cc2eOd6u.o /usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc.a /usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc_eh.a -lc /usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc.a /usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc_eh.a /usr/lib/gcc-cross/arm-linux-gnueabi/9/crtendS.o /musl-armv5te/lib/crtn.o
/usr/lib/gcc-cross/arm-linux-gnueabi/9/../../../../arm-linux-gnueabi/bin/ld: /usr/lib/gcc-cross/arm-linux-gnueabi/9/libgcc.a(_dvmd_lnx.o): in function `__aeabi_ldiv0':
(.text+0x8): undefined reference to `raise'
collect2: error: ld returned 1 exit status
