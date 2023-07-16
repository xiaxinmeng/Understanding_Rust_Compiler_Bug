console
$ ldd build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-110e2444bfbc1fba.so
	linux-vdso.so.1 (0x00007fff511fb000)
	libstd-6593c2e4e017f5fb.so => /home/michael/Documents/forks/rust/build/x86_64-unknown-linux-gnu/stage1/lib/../lib/libstd-6593c2e4e017f5fb.so (0x00007fd8e30e2000)
	libLLVM-11-rust-1.51.0-nightly.so => /home/michael/Documents/forks/rust/build/x86_64-unknown-linux-gnu/stage1/lib/../lib/libLLVM-11-rust-1.51.0-nightly.so (0x00007fd8de6a1000)
	libstdc++.so.6 => /usr/lib/libstdc++.so.6 (0x00007fd8de494000)
	libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007fd8de47a000)
	libpthread.so.0 => /usr/lib/libpthread.so.0 (0x00007fd8de458000)
	libm.so.6 => /usr/lib/libm.so.6 (0x00007fd8de312000)
	libdl.so.2 => /usr/lib/libdl.so.2 (0x00007fd8de30a000)
	libc.so.6 => /usr/lib/libc.so.6 (0x00007fd8de141000)
	/usr/lib64/ld-linux-x86-64.so.2 (0x00007fd8e776f000)
	libz.so.1 => /usr/lib/libz.so.1 (0x00007fd8de127000)
	librt.so.1 => /usr/lib/librt.so.1 (0x00007fd8de11c000)
