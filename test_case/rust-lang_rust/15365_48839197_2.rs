
[root@vds bin]# ldd rustc
./rustc: /lib64/libc.so.6: version `GLIBC_2.6' not found (required by ./rustc)
        linux-vdso.so.1 =>  (0x00007fff39192000)
        libpthread.so.0 => /lib64/libpthread.so.0 (0x00007f3c71979000)
        librt.so.1 => /lib64/librt.so.1 (0x00007f3c7176f000)
        libdl.so.2 => /lib64/libdl.so.2 (0x00007f3c7156b000)
        libm.so.6 => /lib64/libm.so.6 (0x00007f3c712e8000)
        libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x00007f3c710d9000)
        libc.so.6 => /lib64/libc.so.6 (0x00007f3c70d80000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f3c71baa000)
