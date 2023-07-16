
$ rustc -Z use-lld --target x86_64-unknown-linux-musl zero.rs
duplicate symbol: __tls_get_addr in /build/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-0a08e49eef7bbac5.rlib(__tls_get_addr.lo) and (internal)
