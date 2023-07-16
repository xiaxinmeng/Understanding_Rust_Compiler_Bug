
goodnight % for i in rust rust-vanilla; do du -hs $i/x86_64-unknown-linux-gnu/stage2/lib/librustc_llvm-*.so; done
42M     rust/x86_64-unknown-linux-gnu/stage2/lib/librustc_llvm-4e7c5e5c.so
40M     rust-vanilla/x86_64-unknown-linux-gnu/stage2/lib/librustc_llvm-4e7c5e5c.so
goodnight % for i in rust rust-vanilla; do du -hs $i/x86_64-unknown-linux-gnu/stage2/bin/rustc; done
8.0K    rust/x86_64-unknown-linux-gnu/stage2/bin/rustc
8.0K    rust-vanilla/x86_64-unknown-linux-gnu/stage2/bin/rustc
