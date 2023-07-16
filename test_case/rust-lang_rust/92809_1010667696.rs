
~/Temp> rustc -Cprefer-dynamic=yes -Cprefer-dynamic=no -
fn main() {}
~/Temp> ldd rust_out
        linux-vdso.so.1 (0x00007ffecc5f7000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f7dfb1e9000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f7dfb1c6000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f7dfb1c0000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f7dfafce000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f7dfb259000)
~/Temp> rustc -Cprefer-dynamic=yes -
fn main() {}
~/Temp> ldd rust_out
        linux-vdso.so.1 (0x00007ffc60bd4000)
        libstd-13e2ae73269b4206.so => not found
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fccbf6b5000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fccbf4c3000)
        /lib64/ld-linux-x86-64.so.2 (0x00007fccbf6e0000)
