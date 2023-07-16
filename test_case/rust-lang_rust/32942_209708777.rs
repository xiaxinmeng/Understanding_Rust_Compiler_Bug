
compile:

clang++ -std=c++11 -nostdlib -mstackrealign -g -I/usr/include/c++/v1/ -I/usr/include/libcxxabi -I/usr/lib/llvm-3.4/include  -DNDEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -g -O2 -fomit-frame-pointer -fvisibility-inlines-hidden -fno-exceptions -fPIC -Woverloaded-virtual -Wcast-qual -c test2.cc

link:

/usr/bin/ld -z relro --hash-style=gnu --build-id --eh-frame-hdr -m elf_x86_64 -dynamic-linker /lib64/ld-linux-x86-64.so.2 -o test2 /usr/bin/../lib/gcc/x86_64-linux-gnu/4.8/../../../x86_64-linux-gnu/crt1.o /usr/bin/../lib/gcc/x86_64-linux-gnu/4.8/../../../x86_64-linux-gnu/crti.o /usr/bin/../lib/gcc/x86_64-linux-gnu/4.8/crtbegin.o -L/usr/lib/llvm-3.4/lib -L/usr/lib/x86_64-linux-gnu/ -L/lib64 -L/lib -L/usr/lib test2.o -lc++ -lc++abi -lunwind -lc -lpthread -lffi -ltinfo -ldl -lm /usr/bin/../lib/gcc/x86_64-linux-gnu/4.8/crtend.o /usr/bin/../lib/gcc/x86_64-linux-gnu/4.8/../../../x86_64-linux-gnu/crtn.o

ldd:

        linux-vdso.so.1 =>  (0x00007fff5d7fe000)
        libc++.so.1 => /usr/lib/x86_64-linux-gnu/libc++.so.1 (0x00007fac64095000)
        libc++abi.so.1 => /usr/lib/x86_64-linux-gnu/libc++abi.so.1 (0x00007fac63e46000)
        libunwind.so.8 => /usr/lib/x86_64-linux-gnu/libunwind.so.8 (0x00007fac63c2a000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fac63864000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007fac63646000)
        libffi.so.6 => /usr/lib/x86_64-linux-gnu/libffi.so.6 (0x00007fac6343d000)
        libtinfo.so.5 => /lib/x86_64-linux-gnu/libtinfo.so.5 (0x00007fac63214000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007fac63010000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fac62d09000)
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007fac62b01000)
        /lib64/ld-linux-x86-64.so.2 (0x00007fac643ad000)
        liblzma.so.5 => /lib/x86_64-linux-gnu/liblzma.so.5 (0x00007fac628df000)
