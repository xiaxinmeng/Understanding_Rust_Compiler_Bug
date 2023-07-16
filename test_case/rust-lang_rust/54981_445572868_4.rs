
$ strip target/x86_64-unknown-linux-gnu/release/tiny -o tiny

$ ldd tiny 
	linux-vdso.so.1 (0x000003fdde54f000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x000003d847699000)
	/lib64/ld-linux-x86-64.so.2 (0x0000031694ddc000)

$ ls -lh tiny 
-rwxrwxr-x 1 vi vi 6.0K Dec 10 00:21 tiny
