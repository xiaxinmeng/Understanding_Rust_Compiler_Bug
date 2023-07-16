
$ ldd target/release/hello-world
	linux-vdso.so.1 =>  (0x00007ffe11e2a000)
	libdl.so.2 => /lib64/libdl.so.2 (0x00007fcf1e55d000)
	librt.so.1 => /lib64/librt.so.1 (0x00007fcf1e355000)
	libpthread.so.0 => /lib64/libpthread.so.0 (0x00007fcf1e139000)
	libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x00007fcf1df23000)
	libc.so.6 => /lib64/libc.so.6 (0x00007fcf1db56000)
	/lib64/ld-linux-x86-64.so.2 (0x00007fcf1e992000)
