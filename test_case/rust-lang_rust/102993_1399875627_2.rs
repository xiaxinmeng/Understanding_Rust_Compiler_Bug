console
# ldd target/debug/tester
	/lib/ld-linux-aarch64.so.1 (0xffffa0b61000)
	libcurl.so.4 => /usr/lib/libcurl.so.4 (0xffffa0ace000)
	libnghttp2.so.14 => /usr/lib/libnghttp2.so.14 (0xffffa0a8d000)
	libssl.so.3 => /lib/libssl.so.3 (0xffffa09e9000)
	libcrypto.so.3 => /lib/libcrypto.so.3 (0xffffa0663000)
	libbrotlidec.so.1 => /usr/lib/libbrotlidec.so.1 (0xffffa0642000)
	libz.so.1 => /lib/libz.so.1 (0xffffa0611000)
	libc.musl-aarch64.so.1 => /lib/ld-linux-aarch64.so.1 (0xffffa0b61000)
	libbrotlicommon.so.1 => /usr/lib/libbrotlicommon.so.1 (0xffffa05e0000)

# ls /lib/ld-linux-aarch64.so.1
ls: /lib/ld-linux-aarch64.so.1: No such file or directory
