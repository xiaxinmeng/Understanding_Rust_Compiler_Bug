
fails   : RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so" cargo +stable build --release
succeeds: RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so" cargo +1.44.1 build --release
fails   : RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so -Cembed-bitcode=yes" cargo +stable build --release

fails   : CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang RUSTFLAGS="-Clinker-plugin-lto -Clink-arg=-flto" cargo +stable build --release
succeeds: CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang RUSTFLAGS="-Clinker-plugin-lto -Clink-arg=-flto" cargo +1.44.1 build --release
fails   : CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang RUSTFLAGS="-Clinker-plugin-lto -Cembed-bitcode=yes -Clink-arg=-flto" cargo +stable build --release

succeeds: RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so -Clink-arg=-fuse-ld=gold" cargo +stable build --release
succeeds: RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so -Clink-arg=-fuse-ld=lld" cargo +stable build --release
succeeds: RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so -Cembed-bitcode=yes -Clink-arg=-fuse-ld=gold" cargo +stable build --release
succeeds: RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so -Cembed-bitcode=yes -Clink-arg=-fuse-ld=lld" cargo +stable build --release

succeeds: CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang RUSTFLAGS="-Clinker-plugin-lto -Clink-arg=-flto -Clink-arg=-fuse-ld=gold" cargo +stable build --release
succeeds: CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang RUSTFLAGS="-Clinker-plugin-lto -Clink-arg=-flto -Clink-arg=-fuse-ld=lld" cargo +stable build --release
succeeds: CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang RUSTFLAGS="-Clinker-plugin-lto -Cembed-bitcode=yes -Clink-arg=-flto -Clink-arg=-fuse-ld=gold" cargo +stable build --release
succeeds: CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang RUSTFLAGS="-Clinker-plugin-lto -Cembed-bitcode=yes -Clink-arg=-flto -Clink-arg=-fuse-ld=lld" cargo +stable build --release
