
$ readelf -d ~/.rustup/toolchains/1.28.0-aarch64-unknown-linux-gnu/bin/rustc | head

Dynamic section at offset 0x1d28 contains 30 entries:
  Tag        Type                         Name/Value
 0x0000000000000001 (NEEDED)             Shared library: [librustc_driver-2d4297d0ae6fcf70.so]
 0x0000000000000001 (NEEDED)             Shared library: [libstd-9a440ef90899818f.so]
 0x0000000000000001 (NEEDED)             Shared library: [libc.so.6]
 0x000000000000000f (RPATH)              Library rpath: [$ORIGIN/../lib]
 0x000000000000000c (INIT)               0xc48
 0x000000000000000d (FINI)               0xf70
 0x0000000000000019 (INIT_ARRAY)         0x11ce0
