
$ readelf -d /nix/store/kp0akwshg31bc5mj9qbfzjswr1sa3xvi-mkl-2021.1.1.52/lib/libmkl_core.so | rg NEEDED
 0x0000000000000001 (NEEDED)             Shared library: [libdl.so.2]
 0x0000000000000001 (NEEDED)             Shared library: [libpthread.so.0]
