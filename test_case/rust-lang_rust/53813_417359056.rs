
$ ldd $(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld
        linux-vdso.so.1 (0x00007ffe30fa4000)
        libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007ff1b4c40000)
        libLLVM-7.so => not found
        libm.so.6 => /usr/lib/libm.so.6 (0x00007ff1b4abb000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007ff1b4aa1000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007ff1b48dd000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007ff1b4c9d000)
