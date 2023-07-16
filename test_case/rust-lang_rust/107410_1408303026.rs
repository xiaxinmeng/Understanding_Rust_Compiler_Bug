sh
$ LD_PRELOAD=/usr/lib/libtcmalloc_minimal.so rustc -Vv
rustc 1.67.0 (fc594f156 2023-01-24) (Arch Linux rust 1:1.67.0-2)
binary: rustc
commit-hash: fc594f15669680fa70d255faec3ca3fb507c3405
commit-date: 2023-01-24
host: x86_64-unknown-linux-gnu
release: 1.67.0
LLVM version: 15.0.7
$ LD_PRELOAD=/usr/lib/libtcmalloc_minimal.so rustc --print sysroot
/usr
