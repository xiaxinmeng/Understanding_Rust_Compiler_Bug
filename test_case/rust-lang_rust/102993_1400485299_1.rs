
$ gcc -static-pie curl.c -lcurl -lssl -lcrypto -lz -lnghttp2 -lbrotlidec -lbrotlicommon -o curl
$ ldd curl
	/lib/ld-musl-aarch64.so.1 (0xffff90fab000)
