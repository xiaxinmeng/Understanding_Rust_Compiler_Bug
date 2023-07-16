
rust-builder-test:~$ cat > curl.c
#include <curl/curl.h>

int main(void) {
  CURL *curl = curl_easy_init();
  return 0;
}
rust-builder-test:~$ gcc curl.c -lcurl -o curl
rust-builder-test:~$ ldd curl
	/lib/ld-musl-aarch64.so.1 (0xffffbef04000)
	libcurl.so.4 => /usr/lib/libcurl.so.4 (0xffffbee50000)
	libc.musl-aarch64.so.1 => /lib/ld-musl-aarch64.so.1 (0xffffbef04000)
	libnghttp2.so.14 => /usr/lib/libnghttp2.so.14 (0xffffbee0f000)
	libssl.so.3 => /lib/libssl.so.3 (0xffffbed6b000)
	libcrypto.so.3 => /lib/libcrypto.so.3 (0xffffbe9e5000)
	libbrotlidec.so.1 => /usr/lib/libbrotlidec.so.1 (0xffffbe9c4000)
	libz.so.1 => /lib/libz.so.1 (0xffffbe993000)
	libbrotlicommon.so.1 => /usr/lib/libbrotlicommon.so.1 (0xffffbe962000)
