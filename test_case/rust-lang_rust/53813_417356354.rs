 console
$ ldd $(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld
        linux-vdso.so.1 (0x00007ffe1cd74000)
        libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007fab8c935000)
        libLLVM-7.so => /home/japaric/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/libLLVM-7.so (0x00007fab88ecb000)
        libm.so.6 => /usr/lib/libm.so.6 (0x00007fab88d46000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007fab88d2c000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007fab88b68000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007fab8c984000)
        librt.so.1 => /usr/lib/librt.so.1 (0x00007fab88b5c000)
        libdl.so.2 => /usr/lib/libdl.so.2 (0x00007fab88b57000)
        libstdc++.so.6 => /usr/lib/libstdc++.so.6 (0x00007fab889c8000)

$ $(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld -flavor gnu -version
LLD 7.0.0 (compatible with GNU linkers)

$ rustc -V
rustc 1.30.0-nightly (02cb8f2a4 2018-08-29)
