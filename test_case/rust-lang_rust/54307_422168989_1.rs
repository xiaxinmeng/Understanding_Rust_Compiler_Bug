
$ readelf -d ~/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/bin/rustc | head

Dynamic section at offset 0x1d28 contains 30 entries:
  Tag        Type                         Name/Value
 0x0000000000000001 (NEEDED)             Shared library: [librustc_driver-329ba9d6c8478143.so]
 0x0000000000000001 (NEEDED)             Shared library: [libstd-6189f2be2c0d28d3.so]
 0x0000000000000001 (NEEDED)             Shared library: [libc.so.6]
 0x000000000000000f (RPATH)              Library rpath: [$ORIGIN/../lib]
 0x000000000000000c (INIT)               0xc60
 0x000000000000000d (FINI)               0xf90
 0x0000000000000019 (INIT_ARRAY)         0x11ce0
