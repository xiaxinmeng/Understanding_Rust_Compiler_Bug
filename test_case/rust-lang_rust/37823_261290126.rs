
Using built-in specs.
COLLECT_GCC=/usr/mips64el-unknown-linux-gnu/gcc-bin/5.4.0/gcc
COLLECT_LTO_WRAPPER=/usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/lto-wrapper
gcc (Gentoo 5.4.0 p1.0, pie-0.6.5) 5.4.0
Copyright (C) 2015 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.


Target: mips64el-unknown-linux-gnu
Configured with: /var/tmp/portage/sys-devel/gcc-5.4.0/work/gcc-5.4.0/configure --host=mips64el-unknown-linux-gnu --build=mips64el-unknown-linux-gnu --prefix=/usr --bindir=/usr/mips64el-unknown-linux-gnu/gcc-bin/5.4.0 --includedir=/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/include --datadir=/usr/share/gcc-data/mips64el-unknown-linux-gnu/5.4.0 --mandir=/usr/share/gcc-data/mips64el-unknown-linux-gnu/5.4.0/man --infodir=/usr/share/gcc-data/mips64el-unknown-linux-gnu/5.4.0/info --with-gxx-include-dir=/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/include/g++-v5 --with-python-dir=/share/gcc-data/mips64el-unknown-linux-gnu/5.4.0/python --enable-languages=c,c++,fortran --enable-obsolete --enable-secureplt --disable-werror --with-system-zlib --disable-nls --enable-checking=release --with-bugurl=https://bugs.gentoo.org/ --with-pkgversion='Gentoo 5.4.0 p1.0, pie-0.6.5' --enable-libstdcxx-time --enable-shared --enable-threads=posix --enable-__cxa_atexit --enable-clocale=gnu --enable-multilib --disable-altivec --disable-fixed-point --with-abi=64 --disable-libgcj --enable-libgomp --disable-libmudflap --disable-libssp --disable-libcilkrts --disable-libmpx --disable-vtable-verify --disable-libvtv --enable-lto --with-isl --disable-isl-version-check --disable-libsanitizer
Thread model: posix
gcc version 5.4.0 (Gentoo 5.4.0 p1.0, pie-0.6.5) 
COLLECT_GCC_OPTIONS='--version' '-v' '-mabi=64' '-mllsc' '-mno-shared' '-EL'
 /usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/cc1 -quiet -v help-dummy -mel -quiet -dumpbase help-dummy -mabi=64 -mllsc -mno-shared -auxbase help-dummy -version --version -fstack-protector-strong -o /tmp/ccS1TELP.s
GNU C11 (Gentoo 5.4.0 p1.0, pie-0.6.5) version 5.4.0 (mips64el-unknown-linux-gnu)
    compiled by GNU C version 5.4.0, GMP version 6.1.1, MPFR version 3.1.4, MPC version 1.0.3
GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
COLLECT_GCC_OPTIONS='--version' '-v' '-mabi=64' '-mllsc' '-mno-shared' '-EL'
 /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../mips64el-unknown-linux-gnu/bin/as -v -EL -O1 -no-mdebug -mabi=64 -mno-shared -KPIC --version -o /tmp/cciWbyMv.o /tmp/ccS1TELP.s
GNU assembler version 2.26.1 (mips64el-unknown-linux-gnu) using BFD version (Gentoo 2.26.1 p1.0) 2.26.1
GNU assembler (Gentoo 2.26.1 p1.0) 2.26.1
Copyright (C) 2015 Free Software Foundation, Inc.
This program is free software; you may redistribute it under the terms of
the GNU General Public License version 3 or later.
This program has absolutely no warranty.
This assembler was configured for a target of `mips64el-unknown-linux-gnu'.
COMPILER_PATH=/usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/:/usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/:/usr/libexec/gcc/mips64el-unknown-linux-gnu/:/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/:/usr/lib/gcc/mips64el-unknown-linux-gnu/:/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../mips64el-unknown-linux-gnu/bin/
LIBRARY_PATH=/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/:/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64/:/lib64/../lib64/:/usr/lib64/../lib64/:/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../mips64el-unknown-linux-gnu/lib/:/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../:/lib64/:/usr/lib64/
COLLECT_GCC_OPTIONS='--version' '-v' '-mabi=64' '-mllsc' '-mno-shared' '-EL'
 /usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/collect2 -plugin /usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/liblto_plugin.so -plugin-opt=/usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/lto-wrapper -plugin-opt=-fresolution=/tmp/ccatiVca.res -plugin-opt=-pass-through=-lgcc -plugin-opt=-pass-through=-lgcc_s -plugin-opt=-pass-through=-lc -plugin-opt=-pass-through=-lgcc -plugin-opt=-pass-through=-lgcc_s --eh-frame-hdr -EL -dynamic-linker /lib64/ld.so.1 -melf64ltsmip --version /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64/crt1.o /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64/crti.o /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/crtbegin.o -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0 -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64 -L/lib64/../lib64 -L/usr/lib64/../lib64 -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../mips64el-unknown-linux-gnu/lib -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../.. -L/lib64 -L/usr/lib64 /tmp/cciWbyMv.o -lgcc --as-needed -lgcc_s --no-as-needed -lc -lgcc --as-needed -lgcc_s --no-as-needed /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/crtend.o /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64/crtn.o
collect2 version 5.4.0
/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../mips64el-unknown-linux-gnu/bin/ld -plugin /usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/liblto_plugin.so -plugin-opt=/usr/libexec/gcc/mips64el-unknown-linux-gnu/5.4.0/lto-wrapper -plugin-opt=-fresolution=/tmp/ccatiVca.res -plugin-opt=-pass-through=-lgcc -plugin-opt=-pass-through=-lgcc_s -plugin-opt=-pass-through=-lc -plugin-opt=-pass-through=-lgcc -plugin-opt=-pass-through=-lgcc_s --eh-frame-hdr -EL -dynamic-linker /lib64/ld.so.1 -melf64ltsmip --version /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64/crt1.o /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64/crti.o /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/crtbegin.o -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0 -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64 -L/lib64/../lib64 -L/usr/lib64/../lib64 -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../mips64el-unknown-linux-gnu/lib -L/usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../.. -L/lib64 -L/usr/lib64 /tmp/cciWbyMv.o -lgcc --as-needed -lgcc_s --no-as-needed -lc -lgcc --as-needed -lgcc_s --no-as-needed /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/crtend.o /usr/lib/gcc/mips64el-unknown-linux-gnu/5.4.0/../../../../lib64/crtn.o
GNU ld (Gentoo 2.26.1 p1.0) 2.26.1
Copyright (C) 2015 Free Software Foundation, Inc.
This program is free software; you may redistribute it under the terms of
the GNU General Public License version 3 or (at your option) a later version.
This program has absolutely no warranty.
