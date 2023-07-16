
rustup update nightly
cargo install xargo
cargo install sgxs-tools --version 0.6.0-rc1
cargo install fortanix-sgx-tools --version 0.1.0-rc1
# Soon, instead: cargo install fortanix-sgx-tools --git https://github.com/fortanix/rust-sgx
cargo new --bin sgxtest
cd sgxtest
echo '[target.x86_64-fortanix-unknown-sgx.dependencies.std]' > Xargo.toml
xargo build --target x86_64-fortanix-unknown-sgx
ftxsgx-elf2sgxs target/x86_64-fortanix-unknown-sgx/debug/sgxtest --heap-size 0x20000 --ssaframesize 1 --stack-size 0x20000 --threads 1 --debug
sgxs-append -i target/x86_64-fortanix-unknown-sgx/debug/sgxtest.sgxs
ftxsgx-runner target/x86_64-fortanix-unknown-sgx/debug/sgxtest.sgxs
