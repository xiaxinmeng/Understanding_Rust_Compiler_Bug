
[walter@cuddles linking-example]$ ldd target/release/linking-example
	linux-vdso.so.1 (0x00007ffecbdae000)
	libstd-62c4894b82797b30.so => not found                     <========# Looking at this entry
	libc.so.6 => /usr/lib/libc.so.6 (0x00007f53b4da1000)
	/lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007f53b4fbc000)
