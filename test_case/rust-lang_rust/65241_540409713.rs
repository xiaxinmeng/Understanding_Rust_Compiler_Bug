
~/rust $ ./x.py build --stage 1
~/rust $ mkdir -p build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/src
~/rust $ ln -sT $PWD build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/src/rust
~/rust $ rustup toolchain link stage1 build/x86_64-unknown-linux-gnu/stage1

# Testing on tokio: 
~/tokio $ cargo clean
~/tokio $ env CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS=-Zsanitizer=thread cargo +stage1 test --target x86_64-unknown-linux-gnu
~/tokio $ cargo clean
~/tokio $ env CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS=-Zsanitizer=thread cargo +stage1 -Zbuild-std test --target x86_64-unknown-linux-gnu
