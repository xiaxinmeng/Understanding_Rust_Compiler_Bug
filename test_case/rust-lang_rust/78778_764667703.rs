console
$ ldd build/x86_64-unknown-linux-gnu/stage0-tools-bin/miri
	linux-vdso.so.1 (0x00007ffeac5f7000)
	librustc_driver-110e2444bfbc1fba.so => not found
	libstd-6593c2e4e017f5fb.so => not found
	libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f0f3d890000)
	libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007f0f3d86e000)
	libm.so.6 => /usr/lib/libm.so.6 (0x00007f0f3d728000)
	libc.so.6 => /usr/lib/libc.so.6 (0x00007f0f3d55f000)
	/lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007f0f3dced000)
