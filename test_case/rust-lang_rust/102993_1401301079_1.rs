
$ gcc -c -o curl.o curl.c
$ gcc -o curl curl.o -Wl,-Bdynamic -lcurl -static
$ ldd curl
	/lib/ld-linux-aarch64.so.1 (0xffffb387c000)
	libcurl.so.4 => /usr/lib/libcurl.so.4 (0xffffb37e9000)
	libc.musl-aarch64.so.1 => /lib/ld-linux-aarch64.so.1 (0xffffb387c000)
	libnghttp2.so.14 => /usr/lib/libnghttp2.so.14 (0xffffb37a8000)
	libssl.so.3 => /lib/libssl.so.3 (0xffffb3704000)
	libcrypto.so.3 => /lib/libcrypto.so.3 (0xffffb337e000)
	libbrotlidec.so.1 => /usr/lib/libbrotlidec.so.1 (0xffffb335d000)
	libz.so.1 => /lib/libz.so.1 (0xffffb332c000)
	libbrotlicommon.so.1 => /usr/lib/libbrotlicommon.so.1 (0xffffb32fb000)
