shell
$ export env RUSTFLAGS="-Zsanitizer=memory -Zsanitizer-memory-track-origins"
$ cargo -Zbuild-std run --target x86_64-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/x86_64-unknown-linux-gnu/debug/foo`
Hello, world!
$ cargo -Zbuild-std r --target x86_64-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/x86_64-unknown-linux-gnu/debug/foo`
Uninitialized bytes in __interceptor_memchr at offset 0 inside [0x701000000000, 4)
==5321==WARNING: MemorySanitizer: use-of-uninitialized-value
...
