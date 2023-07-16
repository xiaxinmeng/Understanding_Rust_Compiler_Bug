
$ rustc -C target-feature=-crt-static hello_world.rs
$ ./hello_world
Hello, world!
$ ldd hello_world
        /lib/ld-musl-x86_64.so.1 (0x2ecbf67a000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x2ecbf240000)
        libc.musl-x86_64.so.1 => /lib/ld-musl-x86_64.so.1 (0x2ecbf67a000)
