
$ readelf -d /nix/store/kp0akwshg31bc5mj9qbfzjswr1sa3xvi-mkl-2021.1.1.52/lib/libiomp5.so | rg NEEDED
 0x0000000000000001 (NEEDED)             Shared library: [libgcc_s.so.1]
 0x0000000000000001 (NEEDED)             Shared library: [libpthread.so.0]
 0x0000000000000001 (NEEDED)             Shared library: [libc.so.6]
 0x0000000000000001 (NEEDED)             Shared library: [ld-linux-x86-64.so.2]
 0x0000000000000001 (NEEDED)             Shared library: [libdl.so.2]
