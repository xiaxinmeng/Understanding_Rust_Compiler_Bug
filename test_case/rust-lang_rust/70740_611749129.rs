console
❯ RUSTC_LOG=debug rustc --target x86_64-unknown-linux-musl -C link-arg='-fuse-ld=gold' test-aslr.rs
[…]
[INFO  rustc_codegen_ssa::back::link] preparing Executable to "test-aslr"
[INFO  rustc_codegen_ssa::back::link] "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--eh-frame-hdr" "-m64" "-nostdlib" "/tmp/rust-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/rcrt1.o" […]
[INFO  rustc_codegen_ssa::back::link] linker output: "/usr/bin/ld.gold: --no-dynamic-linker: unknown option\n/usr/bin/ld.gold: use the --help option for usage information\ncollect2: error: ld returned 1 exit status\n"
[WARN  rustc_codegen_ssa::back::link] Linker does not support -static-pie command line option. Retrying without.
[INFO  rustc_codegen_ssa::back::link] new cmdline: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--eh-frame-hdr" "-m64" "-nostdlib" "/tmp/rust-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/crt1.o" […]

❯ ./test-aslr --test-aslr
FAIL: ./test-aslr most likely no ASLR

❯ file test-aslr
test-aslr: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, BuildID[sha1]=c2cfcd855bd6bd0d8c6a0ebf6bb6e2ca6f33966e, with debug_info, not stripped
