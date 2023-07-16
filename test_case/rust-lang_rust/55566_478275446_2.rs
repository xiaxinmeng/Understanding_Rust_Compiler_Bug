
cc -Wl,--as-needed -Wl,-z,noexecstack -Wl,--eh-frame-hdr -m64 -nostdlib \
    /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/crt1.o \
    /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/crti.o \
    /tmp/foo/linkage-bug/target/debug/deps/linkage_bug.o \
    -Wl,--gc-sections -no-pie -Wl,-zrelro -Wl,-znow -nodefaultlibs \
    -L /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib \
    -L /tmp/foo/linkage-bug/target/debug/deps -L /usr/lib \
    -Wl,-Bstatic \
    /tmp/foo/linkage-bug/target/debug/deps/libcurl-d49997a6f4108f3d.rlib \
    /tmp/foo/linkage-bug/target/debug/deps/libopenssl_probe-de3c2a56f5a03bc7.rlib \
    /tmp/foo/linkage-bug/target/debug/deps/libsocket2-85038c1da7d16bd8.rlib \
    /tmp/foo/linkage-bug/target/debug/deps/libcfg_if-1b5a1417bb6c33aa.rlib \
    /tmp/foo/linkage-bug/target/debug/deps/libcurl_sys-46f40a16606e3983.rlib \
    /tmp/foo/linkage-bug/target/debug/deps/libopenssl_sys-c13864e6f82dcb29.rlib \
    /tmp/foo/linkage-bug/target/debug/deps/liblibz_sys-4ade5aa497d796a3.rlib \
    /tmp/foo/linkage-bug/target/debug/deps/liblibc-8588740f81bd49b8.rlib \
    -Wl,--start-group \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/libstd-0c4f6012505a64ae.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/libpanic_unwind-ff29f889cb57b5b6.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/libbacktrace_sys-b3e53f8e9f58cb0d.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_demangle-2bc5c9c36bf4533a.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/libunwind-ab23897a9a0c444b.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/liblibc-5f067c62510edb37.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-7d38043a47a71b82.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/librustc_std_workspace_core-8f4dba2468f32197.rlib \
        /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/libcore-1b9655321933a544.rlib \
    -Wl,--end-group \
    /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/libcompiler_builtins-64a0a8f9d3310074.rlib \
    -Wl,-Bdynamic \
    -lcurl -lssl -lcrypto -lz \
    -static \
    /tmp/rust-nightly/lib/rustlib/x86_64-unknown-linux-musl/lib/crtn.o
