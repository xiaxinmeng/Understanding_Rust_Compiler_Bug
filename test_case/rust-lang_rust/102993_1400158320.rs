console
$ RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build
[...]
    Finished dev [unoptimized + debuginfo] target(s) in 10.89s
$ ldd target/debug/tester
	/lib/ld-musl-aarch64.so.1 (0xffff928d0000)
	libcurl.so.4 => /usr/lib/libcurl.so.4 (0xffff927cc000)
	libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0xffff9279b000)
	libc.musl-aarch64.so.1 => /lib/ld-musl-aarch64.so.1 (0xffff928d0000)
	libnghttp2.so.14 => /usr/lib/libnghttp2.so.14 (0xffff9275a000)
	libssl.so.3 => /lib/libssl.so.3 (0xffff926b6000)
	libcrypto.so.3 => /lib/libcrypto.so.3 (0xffff92330000)
	libbrotlidec.so.1 => /usr/lib/libbrotlidec.so.1 (0xffff9230f000)
	libz.so.1 => /lib/libz.so.1 (0xffff922de000)
	libbrotlicommon.so.1 => /usr/lib/libbrotlicommon.so.1 (0xffff922ad000)
