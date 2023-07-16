
$ ln -s $(rustc +nightly --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld ld.lld
$ echo 'fn main() {}' | RUSTC_LOG=info rustc +nightly - -C linker_plugin_lto -C linker=clang -C link-arg=-B. -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--plugin-opt=-lto-embed-bitcode=optimized
