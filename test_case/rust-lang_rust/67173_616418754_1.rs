
$ ldd target/release/hello-world
	linux-vdso.so.1 =>  (0x00007fff6eafb000)
	libdl.so.2 => /lib64/libdl.so.2 (0x00007f7ce0a6e000)
	librt.so.1 => /lib64/librt.so.1 (0x00007f7ce0866000)
	libpthread.so.0 => /lib64/libpthread.so.0 (0x00007f7ce0648000)
	libgcc_s.so.1 => /lib64/libgcc_s.so.1 (0x00007f7ce0432000)
	libc.so.6 => /lib64/libc.so.6 (0x00007f7ce009e000)
	/lib64/ld-linux-x86-64.so.2 (0x000055ed96f5a000)
$ objdump -T target/release/hello-world|grep -i glibc|grep -i "memcpy"
0000000000000000      DF *UND*	0000000000000000  GLIBC_2.2.5 memcpy
